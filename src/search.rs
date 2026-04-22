use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::error::{Error, Result};
use crate::http::IptClient;
use crate::models::Torrent;
use crate::utils::{clean_text, parse_int};

static TABLE_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("table#torrents").expect("valid selector"));
static COL_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("td").expect("valid selector"));
static IMG_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("img").expect("valid selector"));
static NAME_LINK_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a[href^=\"/t/\"]").expect("valid selector"));
static DL_LINK_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a[href*=\"download.php\"]").expect("valid selector"));

static ID_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^/t/(\d+)$").expect("valid regex"));
static AGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([\d.]+\s+\w+\s+ago)").expect("valid regex"));

fn resolve_sort_field(sort: &str) -> Option<&'static str> {
    match sort {
        "seeders" => Some("seeders"),
        "leechers" => Some("leechers"),
        "size" => Some("size"),
        "downloads" => Some("completed"),
        "name" => Some("name"),
        "age" => Some("age"),
        _ => None,
    }
}

pub fn search(
    client: &IptClient,
    query: &str,
    limit: usize,
    sort: Option<&str>,
) -> Result<Vec<Torrent>> {
    let mut params = vec![
        ("q".to_string(), query.to_string()),
        ("qf".to_string(), "".to_string()),
    ];
    if let Some(sort) = sort {
        let Some(mapped) = resolve_sort_field(sort) else {
            let valid = "seeders, leechers, size, downloads, name, age".to_string();
            return Err(Error::InvalidSortField(sort.to_string(), valid));
        };
        params.push(("o".to_string(), mapped.to_string()));
    }

    let response = client.get("/t").query(&params).send()?.error_for_status()?;
    let html = response.text()?;
    parse_results_with_base(&html, limit, client.base_url())
}

fn text_of(el: &ElementRef<'_>) -> String {
    clean_text(&el.text().collect::<Vec<_>>().join(" "))
}

fn has_freeleech(name_col: &ElementRef<'_>) -> bool {
    for node in name_col.descendants() {
        let Some(el) = ElementRef::wrap(node) else {
            continue;
        };

        if let Some(class_name) = el.value().attr("class") {
            let lowered = class_name.to_ascii_lowercase();
            if lowered.contains("fl") || lowered.contains("freeleech") {
                return true;
            }
        }

        let txt = text_of(&el).to_ascii_lowercase();
        if txt.contains("freeleech") || txt.contains("free leech") {
            return true;
        }
    }

    false
}

pub fn parse_results(html: &str, limit: usize) -> Result<Vec<Torrent>> {
    parse_results_with_base(html, limit, crate::http::BASE_URL)
}

pub fn parse_results_with_base(html: &str, limit: usize, base_url: &str) -> Result<Vec<Torrent>> {
    let lower = html.to_ascii_lowercase();
    let doc = Html::parse_document(html);

    let Some(table) = doc.select(&TABLE_SEL).next() else {
        if lower.contains("sign in") {
            return Err(Error::NotLoggedIn);
        }
        return Err(Error::UnexpectedSearchLayout);
    };

    let mut out = Vec::new();
    let mut seen_ids = HashSet::new();

    for name_link in table.select(&NAME_LINK_SEL) {
        if out.len() >= limit {
            break;
        }

        let Some(href) = name_link.value().attr("href") else {
            continue;
        };
        let Some(caps) = ID_RE.captures(href) else {
            continue;
        };
        let torrent_id: i64 = caps[1].parse().unwrap_or_default();
        if torrent_id <= 0 || !seen_ids.insert(torrent_id) {
            continue;
        }

        let Some(row) = name_link
            .ancestors()
            .find_map(|node| ElementRef::wrap(node).filter(|el| el.value().name() == "tr"))
        else {
            continue;
        };

        let cols: Vec<_> = row.select(&COL_SEL).collect();
        if cols.len() < 5 {
            continue;
        }

        let name_col_idx = cols
            .iter()
            .position(|col| col.select(&NAME_LINK_SEL).next().is_some())
            .unwrap_or(1.min(cols.len() - 1));

        let name_col = &cols[name_col_idx];

        let category = cols[0]
            .select(&IMG_SEL)
            .next()
            .and_then(|img| img.value().attr("alt"))
            .map(clean_text)
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| "Unknown".to_string());

        let name = text_of(&name_link);

        let added = AGE_RE
            .captures(&text_of(name_col))
            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_default();

        let freeleech = has_freeleech(name_col);

        let download_url = row
            .select(&DL_LINK_SEL)
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(|href| to_absolute_download_url(href, base_url))
            .unwrap_or_else(|| {
                format!("{base_url}/download.php/{torrent_id}/{torrent_id}.torrent")
            });

        let size = cols
            .get(cols.len().saturating_sub(4))
            .map(text_of)
            .unwrap_or_default();
        let downloads = cols
            .get(cols.len().saturating_sub(3))
            .map(text_of)
            .map(|s| parse_int(&s))
            .unwrap_or_default();
        let seeders = cols
            .get(cols.len().saturating_sub(2))
            .map(text_of)
            .map(|s| parse_int(&s))
            .unwrap_or_default();
        let leechers = cols
            .get(cols.len().saturating_sub(1))
            .map(text_of)
            .map(|s| parse_int(&s))
            .unwrap_or_default();

        out.push(Torrent {
            id: torrent_id,
            name,
            category,
            size,
            seeders,
            leechers,
            downloads,
            added,
            freeleech,
            download_url,
        });
    }

    Ok(out)
}

fn to_absolute_download_url(href: &str, base_url: &str) -> String {
    if href.starts_with("http://") || href.starts_with("https://") {
        href.to_string()
    } else {
        format!("{}/{}", base_url, href.trim_start_matches('/'))
    }
}

#[cfg(test)]
mod tests {
    use super::parse_results;
    use crate::test_fixtures::{SEARCH_HTML, SEARCH_HTML_EMPTY, SEARCH_HTML_LOGGED_OUT};

    #[test]
    fn parse_results_returns_all_rows() {
        let results = parse_results(SEARCH_HTML, 25).unwrap();
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn parse_movie_fields() {
        let results = parse_results(SEARCH_HTML, 25).unwrap();
        let movie = &results[0];
        assert_eq!(movie.id, 111_222);
        assert!(movie.name.contains("Blade Runner"));
        assert_eq!(movie.category, "Movie/HD");
        assert_eq!(movie.size, "55.3 GB");
        assert_eq!(movie.seeders, 987);
        assert_eq!(movie.leechers, 12);
        assert_eq!(movie.downloads, 1_234);
        assert!(!movie.freeleech);
    }

    #[test]
    fn parse_limit_is_respected() {
        let results = parse_results(SEARCH_HTML, 1).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 111_222);
    }

    #[test]
    fn parse_empty_table_returns_empty() {
        let results = parse_results(SEARCH_HTML_EMPTY, 25).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn parse_missing_table_returns_layout_error() {
        let err = parse_results("<html><body>no table here</body></html>", 25).unwrap_err();
        assert!(err.to_string().contains("unexpected search page layout"));
    }

    #[test]
    fn parse_logged_out_returns_error() {
        let err = parse_results(SEARCH_HTML_LOGGED_OUT, 25).unwrap_err();
        assert!(err.to_string().contains("not logged in"));
    }

    #[test]
    fn freeleech_detected() {
        let results = parse_results(SEARCH_HTML, 25).unwrap();
        assert!(!results[0].freeleech);
        assert!(!results[1].freeleech);
        assert!(results[2].freeleech);
    }

    #[test]
    fn parse_modern_layout_row_with_extra_links() {
        let html = r#"<!DOCTYPE html><html><body>
<table id="torrents" class="t1">
  <thead>
    <tr><th>Type</th><th>Name</th><th>B</th><th>D</th><th>C</th><th>SZ</th><th>DL</th><th>SE</th><th>LE</th></tr>
  </thead>
  <tbody>
    <tr>
      <td><img alt="Appz"></td>
      <td class="al">
        <a class="b hv" href="/t/7276894">GitLab Enterprise v18 9 2 Ubuntu 24 04 Linux x64</a>
        <div class="sub">1.2 months ago by TvTeam</div>
      </td>
      <td><a href="/t/7276894?bookmark">Bookmark</a></td>
      <td><a href="/download.php/7276894/file.torrent">Download</a></td>
      <td><a href="/t/7276894?page=0#startcomments">0</a></td>
      <td>1.42 GB</td>
      <td>20</td>
      <td>4</td>
      <td>0</td>
    </tr>
  </tbody>
</table>
</body></html>"#;

        let results = parse_results(html, 25).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 7_276_894);
        assert_eq!(results[0].category, "Appz");
        assert_eq!(results[0].downloads, 20);
        assert_eq!(results[0].seeders, 4);
        assert_eq!(results[0].leechers, 0);
        assert_eq!(results[0].added, "1.2 months ago");
    }
}
