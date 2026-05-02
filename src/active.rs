use std::sync::LazyLock;

use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::error::{Error, Result};
use crate::http::IptClient;
use crate::models::{ActiveTorrent, ActiveTorrents};
use crate::utils::clean_text;

static TABLE_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("table.t1").expect("valid selector"));
static ROW_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("tr").expect("valid selector"));
static COL_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("td").expect("valid selector"));
static LINK_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a").expect("valid selector"));

static TORRENT_ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[?&]t=(\d+)").expect("valid regex"));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Seeding,
    Leeching,
}

pub fn active(client: &IptClient) -> Result<ActiveTorrents> {
    let response = client.get("/peers").send()?.error_for_status()?;
    let html = response.text()?;
    parse_active(&html)
}

fn text_of(el: &ElementRef<'_>) -> String {
    clean_text(&el.text().collect::<Vec<_>>().join(" "))
}

fn parse_torrent_id(row: &ElementRef<'_>) -> Option<i64> {
    row.select(&LINK_SEL)
        .filter_map(|a| a.value().attr("href"))
        .find_map(|href| {
            TORRENT_ID_RE
                .captures(href)
                .and_then(|caps| caps.get(1))
                .and_then(|m| m.as_str().parse::<i64>().ok())
        })
}

pub fn parse_active(html: &str) -> Result<ActiveTorrents> {
    let lower = html.to_ascii_lowercase();
    let doc = Html::parse_document(html);

    let Some(table) = doc.select(&TABLE_SEL).next() else {
        if lower.contains("sign in") {
            return Err(Error::NotLoggedIn);
        }
        return Err(Error::UnexpectedPeersLayout);
    };

    let mut seeding = Vec::new();
    let mut leeching = Vec::new();
    let mut section = Section::Seeding;
    let mut saw_section_marker = false;
    let mut parsed_row_count = 0usize;

    for row in table.select(&ROW_SEL) {
        let cols: Vec<_> = row.select(&COL_SEL).collect();
        if cols.is_empty() {
            continue;
        }

        if cols.len() == 1 {
            let marker = text_of(&cols[0]).to_ascii_lowercase();
            if marker.contains("seeders") {
                section = Section::Seeding;
                saw_section_marker = true;
            } else if marker.contains("leechers") {
                section = Section::Leeching;
                saw_section_marker = true;
            }
            continue;
        }

        if cols.len() < 9 {
            continue;
        }

        let torrent_col = &cols[0];
        let torrent = text_of(torrent_col);
        if torrent.is_empty() || torrent.eq_ignore_ascii_case("torrent") {
            continue;
        }

        let Some(torrent_id) = parse_torrent_id(&row).filter(|id| *id > 0) else {
            continue;
        };

        let percent_idx = cols.len().saturating_sub(8);
        let uploaded_idx = cols.len().saturating_sub(7);
        let upload_rate_idx = cols.len().saturating_sub(6);
        let downloaded_idx = cols.len().saturating_sub(5);
        let download_rate_idx = cols.len().saturating_sub(4);
        let seeding_time_idx = cols.len().saturating_sub(3);
        let user_agent_idx = cols.len().saturating_sub(2);
        let ip_idx = cols.len().saturating_sub(1);

        let entry = ActiveTorrent {
            name: torrent,
            id: torrent_id,
            percent: text_of(&cols[percent_idx]),
            uploaded: text_of(&cols[uploaded_idx]),
            upload_rate: text_of(&cols[upload_rate_idx]),
            downloaded: text_of(&cols[downloaded_idx]),
            download_rate: text_of(&cols[download_rate_idx]),
            seeding_time: text_of(&cols[seeding_time_idx]),
            user_agent: text_of(&cols[user_agent_idx]),
            ip_address: text_of(&cols[ip_idx]),
        };

        match section {
            Section::Seeding => seeding.push(entry),
            Section::Leeching => leeching.push(entry),
        }

        parsed_row_count += 1;
    }

    if !saw_section_marker && parsed_row_count == 0 {
        return Err(Error::UnexpectedPeersLayout);
    }

    Ok(ActiveTorrents { seeding, leeching })
}

#[cfg(test)]
mod tests {
    use super::parse_active;
    use crate::test_fixtures::{PEERS_HTML, SEARCH_HTML_LOGGED_OUT};

    #[test]
    fn parse_active_splits_sections() {
        let parsed = parse_active(PEERS_HTML).unwrap();
        assert_eq!(parsed.seeding.len(), 2);
        assert_eq!(parsed.leeching.len(), 1);
    }

    #[test]
    fn parse_active_extracts_fields() {
        let parsed = parse_active(PEERS_HTML).unwrap();
        let first = &parsed.seeding[0];

        assert_eq!(first.name, "28 Years Later 2025 HDR 2160p WEB h265-ETHEL");
        assert_eq!(first.id, 6_788_723);
        assert_eq!(first.percent, "100%");
        assert_eq!(first.uploaded, "329 MB (16.1 GB)");
        assert_eq!(first.upload_rate, "5.3 MB/s");
        assert_eq!(first.downloaded, "0 B (11.8 GB)");
        assert_eq!(first.download_rate, "0 B/s");
        assert_eq!(first.seeding_time, "7.7 months");
        assert_eq!(first.user_agent, "Transmission/4.1.1");
        assert_eq!(first.ip_address, "74.64.39.221");
    }

    #[test]
    fn parse_logged_out_returns_error() {
        let err = parse_active(SEARCH_HTML_LOGGED_OUT).unwrap_err();
        assert!(err.to_string().contains("not logged in"));
    }

    #[test]
    fn parse_missing_table_returns_layout_error() {
        let err = parse_active("<html><body>no peers table</body></html>").unwrap_err();
        assert!(
            err.to_string()
                .contains("unexpected active peers page layout")
        );
    }

    #[test]
    fn parse_table_without_peer_rows_returns_layout_error() {
        let html = r#"<!DOCTYPE html><html><body>
<table class="t1">
  <tr><th>Torrent</th><th>%</th></tr>
</table>
</body></html>"#;

        let err = parse_active(html).unwrap_err();
        assert!(
            err.to_string()
                .contains("unexpected active peers page layout")
        );
    }

    #[test]
    fn parse_row_without_torrent_id_is_skipped() {
        let html = r#"<!DOCTYPE html><html><body>
<table class="t1">
  <tr><th>Torrent</th><th>User</th><th>%</th><th>Uploaded</th><th>Rate</th><th>Downloaded</th><th>Rate</th><th>Seeding Time</th><th>User Agent</th><th>IP Address</th></tr>
  <tr><td colspan="99">Seeders</td></tr>
  <tr>
    <td><a href="?u=123">Missing torrent id</a></td>
    <td></td><td>100%</td><td>1 GB</td><td>0 B/s</td><td>1 GB</td><td>0 B/s</td><td>1 day</td><td>Transmission/4.1.1</td><td>1.2.3.4</td>
  </tr>
</table>
</body></html>"#;

        let parsed = parse_active(html).unwrap();
        assert!(parsed.seeding.is_empty());
        assert!(parsed.leeching.is_empty());
    }
}
