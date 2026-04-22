use std::sync::LazyLock;

use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::error::{Error, Result};
use crate::http::IptClient;
use crate::models::TorrentInfo;
use crate::utils::{clean_text, parse_int};

static TITLE_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("title").expect("valid selector"));
static TABLE_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("table").expect("valid selector"));
static ROW_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("tr").expect("valid selector"));
static CELL_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("td").expect("valid selector"));
static ELAPSED_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("span.elapsedDate").expect("valid selector"));
static UPLOADER_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a[href^=\"/u/\"]").expect("valid selector"));
static PEER_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a.peer span").expect("valid selector"));
static LINK_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a[href]").expect("valid selector"));
static ANY_LINK_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a").expect("valid selector"));
static DL_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a[href*=\"download.php\"]").expect("valid selector"));
static SIZE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Size:\s*([\d.,]+\s*[KMGT]?B)\s*in\s*(\d+)\s*file").expect("valid regex")
});

pub fn fetch_info(client: &IptClient, torrent_id: i64) -> Result<TorrentInfo> {
    let response = client
        .get(&format!("/t/{torrent_id}"))
        .send()?
        .error_for_status()?;
    let html = response.text()?;
    parse_detail(&html, torrent_id, client.base_url())
}

fn text_of(el: &ElementRef<'_>) -> String {
    clean_text(&el.text().collect::<Vec<_>>().join(" "))
}

#[derive(Default)]
struct ParsedStats {
    size: String,
    uploaded: String,
    uploader: String,
    seeders: i64,
    leechers: i64,
    file_count: i64,
}

#[derive(Default)]
struct ParsedMetadata {
    genre: Vec<String>,
    plot: String,
    actors: Vec<String>,
    imdb_url: String,
    tmdb_url: String,
}

fn parse_name(doc: &Html) -> String {
    doc.select(&TITLE_SEL)
        .next()
        .map(|t| clean_text(&t.text().collect::<Vec<_>>().join(" ")))
        .map(|t| t.split(" - IPTorrents").next().unwrap_or("").to_string())
        .unwrap_or_default()
}

fn parse_stats(table: Option<&ElementRef<'_>>) -> ParsedStats {
    let Some(stats) = table else {
        return ParsedStats::default();
    };

    let mut parsed = ParsedStats::default();
    let stats_text = text_of(stats);

    if let Some(caps) = SIZE_RE.captures(&stats_text) {
        parsed.size = caps
            .get(1)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        parsed.file_count = caps
            .get(2)
            .map(|m| m.as_str().parse::<i64>().unwrap_or_default())
            .unwrap_or_default();
    }

    parsed.uploaded = stats
        .select(&ELAPSED_SEL)
        .next()
        .map(|n| text_of(&n))
        .unwrap_or_default();

    for link in stats.select(&UPLOADER_SEL) {
        let txt = text_of(&link);
        if !txt.is_empty() {
            parsed.uploader = txt;
            break;
        }
    }

    let peers: Vec<_> = stats.select(&PEER_SEL).collect();
    if peers.len() >= 2 {
        parsed.seeders = parse_int(&text_of(&peers[0]));
        parsed.leechers = parse_int(&text_of(&peers[1]));
    }

    parsed
}

fn is_stats_table(table: &ElementRef<'_>) -> bool {
    SIZE_RE.is_match(&text_of(table))
}

fn is_metadata_table(table: &ElementRef<'_>) -> bool {
    for row in table.select(&ROW_SEL) {
        let cells: Vec<_> = row.select(&CELL_SEL).collect();
        if cells.len() < 2 {
            continue;
        }
        let label = text_of(&cells[0]).to_ascii_lowercase();
        if label == "genre" || label == "plot" || label == "actors" {
            return true;
        }

        for a in cells[1].select(&LINK_SEL) {
            if let Some(href) = a.value().attr("href") {
                if href.contains("imdb.com") || href.contains("themoviedb.org") {
                    return true;
                }
            }
        }
    }

    false
}

fn parse_metadata(table: Option<&ElementRef<'_>>) -> ParsedMetadata {
    let Some(desc) = table else {
        return ParsedMetadata::default();
    };

    let mut parsed = ParsedMetadata::default();

    for row in desc.select(&ROW_SEL) {
        let cells: Vec<_> = row.select(&CELL_SEL).collect();
        if cells.len() < 2 {
            continue;
        }

        let label = text_of(&cells[0]).to_ascii_lowercase();
        let value = &cells[1];

        match label.as_str() {
            "genre" => {
                parsed.genre = value
                    .select(&ANY_LINK_SEL)
                    .map(|a| text_of(&a))
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "plot" => {
                parsed.plot = text_of(value);
            }
            "actors" => {
                parsed.actors = value
                    .select(&ANY_LINK_SEL)
                    .map(|a| text_of(&a))
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            _ => {
                for a in value.select(&LINK_SEL) {
                    if let Some(href) = a.value().attr("href") {
                        if href.contains("imdb.com") {
                            parsed.imdb_url = href.to_string();
                        }
                        if href.contains("themoviedb.org") {
                            parsed.tmdb_url = href.to_string();
                        }
                    }
                }
            }
        }
    }

    parsed
}

fn to_absolute_url(base_url: &str, href: &str) -> String {
    if href.starts_with("http://") || href.starts_with("https://") {
        href.to_string()
    } else {
        format!("{}/{}", base_url, href.trim_start_matches('/'))
    }
}

fn parse_download_url(doc: &Html, torrent_id: i64, base_url: &str) -> String {
    doc.select(&DL_SEL)
        .next()
        .and_then(|a| a.value().attr("href"))
        .map(|href| to_absolute_url(base_url, href))
        .unwrap_or_else(|| {
            format!(
                "{}/download.php/{torrent_id}/{torrent_id}.torrent",
                base_url
            )
        })
}

pub fn parse_detail(html: &str, torrent_id: i64, base_url: &str) -> Result<TorrentInfo> {
    let doc = Html::parse_document(html);
    let name = parse_name(&doc);

    let tables: Vec<_> = doc.select(&TABLE_SEL).collect();
    let stats_table = tables.iter().find(|table| is_stats_table(table));
    let metadata_table = tables.iter().find(|table| is_metadata_table(table));

    let stats = parse_stats(stats_table);
    let metadata = parse_metadata(metadata_table);
    let download_url = parse_download_url(&doc, torrent_id, base_url);

    if name.is_empty() || stats.size.is_empty() {
        return Err(Error::UnexpectedDetailLayout);
    }

    Ok(TorrentInfo {
        id: torrent_id,
        name,
        size: stats.size,
        uploaded: stats.uploaded,
        uploader: stats.uploader,
        seeders: stats.seeders,
        leechers: stats.leechers,
        file_count: stats.file_count,
        genre: metadata.genre,
        plot: metadata.plot,
        actors: metadata.actors,
        imdb_url: metadata.imdb_url,
        tmdb_url: metadata.tmdb_url,
        download_url,
    })
}

#[cfg(test)]
mod tests {
    use super::parse_detail;
    use crate::test_fixtures::{INFO_HTML_MOVIE, INFO_HTML_MUSIC, INFO_HTML_SOFTWARE};

    #[test]
    fn parse_movie_details() {
        let info = parse_detail(INFO_HTML_MOVIE, 111_222, "https://iptorrents.com").unwrap();
        assert_eq!(info.id, 111_222);
        assert_eq!(info.name, "Blade Runner 2049 2017 2160p UHD BluRay x265");
        assert_eq!(info.size, "55.3 GB");
        assert_eq!(info.file_count, 3);
        assert_eq!(info.uploader, "uploader_guy");
        assert_eq!(info.uploaded, "3 days ago");
        assert_eq!(info.seeders, 987);
        assert_eq!(info.leechers, 12);
        assert_eq!(info.genre, vec!["Sci-Fi", "Drama"]);
        assert!(info.plot.to_lowercase().contains("blade runner"));
        assert!(info.actors.contains(&"Ryan Gosling".to_string()));
        assert!(info.imdb_url.contains("imdb.com"));
        assert!(info.tmdb_url.contains("themoviedb.org"));
    }

    #[test]
    fn parse_software_details_without_metadata() {
        let info = parse_detail(INFO_HTML_SOFTWARE, 333_444, "https://iptorrents.com").unwrap();
        assert_eq!(info.id, 333_444);
        assert!(info.name.contains("Ubuntu"));
        assert_eq!(info.size, "5.68 GB");
        assert_eq!(info.file_count, 1);
        assert_eq!(info.uploader, "linuxfan");
        assert_eq!(info.seeders, 503);
        assert_eq!(info.leechers, 78);
        assert!(info.genre.is_empty());
        assert!(info.plot.is_empty());
        assert!(info.actors.is_empty());
        assert!(info.imdb_url.is_empty());
        assert!(info.tmdb_url.is_empty());
    }

    #[test]
    fn parse_music_details_with_non_movie_table() {
        let info = parse_detail(INFO_HTML_MUSIC, 555_666, "https://iptorrents.com").unwrap();
        assert!(info.name.contains("Pink Floyd"));
        assert_eq!(info.size, "1.2 GB");
        assert_eq!(info.file_count, 24);
        assert_eq!(info.uploader, "audiophile99");
        assert_eq!(info.seeders, 210);
        assert!(info.genre.is_empty());
        assert!(info.plot.is_empty());
        assert!(info.actors.is_empty());
        assert!(info.imdb_url.is_empty());
        assert!(info.tmdb_url.is_empty());
    }

    #[test]
    fn parse_detail_errors_on_unexpected_layout() {
        let err = parse_detail(
            "<html><body>no detail table</body></html>",
            1,
            "https://iptorrents.com",
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("could not parse torrent details page"));
    }
}
