mod support;

use std::io::Cursor;

use httpmock::Method::GET;
use httpmock::MockServer;
use iptorrents_cli::active::active;
use iptorrents_cli::download::{download_torrent, stream_torrent};
use iptorrents_cli::http::{IptClient, verify_session};
use iptorrents_cli::info::fetch_info;
use iptorrents_cli::models::AuthConfig;
use iptorrents_cli::search::search;
use tempfile::tempdir;

use crate::support::fixtures::{INFO_HTML_MOVIE, PEERS_HTML, SEARCH_HTML, SEARCH_HTML_LOGGED_OUT};

const FAKE_TORRENT_BYTES: &[u8] = b"d8:announce35:http://tracker.example.com/announcee";

fn test_client(base_url: String) -> IptClient {
    IptClient::new(
        AuthConfig {
            uid: "fake_uid".to_string(),
            pass_cookie: "fake_pass".to_string(),
            cf_clearance: Some("fake_cf".to_string()),
        },
        base_url,
    )
    .expect("client should build")
}

#[test]
fn verify_session_ok() {
    let server = MockServer::start();
    let _m = server.mock(|when, then| {
        when.method(GET).path("/t");
        then.status(200).body("ok");
    });

    let client = test_client(server.base_url());
    verify_session(&client).expect("session should verify");
}

#[test]
fn verify_session_rejects_sign_in_page() {
    let server = MockServer::start();
    let _m = server.mock(|when, then| {
        when.method(GET).path("/t");
        then.status(200)
            .body("<html><body>Please sign in to continue</body></html>");
    });

    let client = test_client(server.base_url());
    let err = verify_session(&client).unwrap_err();
    assert!(err.to_string().contains("invalid or expired"));
}

#[test]
fn search_returns_results() {
    let server = MockServer::start();

    let _verify = server.mock(|when, then| {
        when.method(GET).path("/t");
        then.status(200).body(SEARCH_HTML);
    });

    let client = test_client(server.base_url());
    let results = search(&client, "blade runner", 25, None).expect("search should succeed");
    assert_eq!(results.len(), 3);
}

#[test]
fn search_sends_sort_param() {
    let server = MockServer::start();

    let sorted = server.mock(|when, then| {
        when.method(GET)
            .path("/t")
            .query_param("q", "ubuntu")
            .query_param("o", "seeders");
        then.status(200).body(SEARCH_HTML);
    });

    let client = test_client(server.base_url());
    let _ = search(&client, "ubuntu", 25, Some("seeders")).expect("search should succeed");
    sorted.assert();
}

#[test]
fn search_logged_out_errors() {
    let server = MockServer::start();
    let _m = server.mock(|when, then| {
        when.method(GET).path("/t");
        then.status(200).body(SEARCH_HTML_LOGGED_OUT);
    });

    let client = test_client(server.base_url());
    let err = search(&client, "anything", 25, None).unwrap_err();
    assert!(err.to_string().contains("not logged in"));
}

#[test]
fn fetch_info_parses_movie() {
    let server = MockServer::start();
    let _m = server.mock(|when, then| {
        when.method(GET).path("/t/111222");
        then.status(200).body(INFO_HTML_MOVIE);
    });

    let client = test_client(server.base_url());
    let info = fetch_info(&client, 111_222).expect("info should parse");
    assert_eq!(info.id, 111_222);
    assert!(info.name.contains("Blade Runner"));
    assert_eq!(info.seeders, 987);
}

#[test]
fn stream_torrent_writes_bytes() {
    let server = MockServer::start();
    let download_path = "/download.php/111222/file.torrent";
    let _m = server.mock(|when, then| {
        when.method(GET).path(download_path);
        then.status(200)
            .header("content-type", "application/x-bittorrent")
            .body(FAKE_TORRENT_BYTES);
    });

    let client = test_client(server.base_url());
    let mut cursor = Cursor::new(Vec::new());
    stream_torrent(
        &client,
        &format!("{}{}", server.base_url(), download_path),
        &mut cursor,
    )
    .expect("stream should work");
    assert_eq!(cursor.into_inner(), FAKE_TORRENT_BYTES);
}

#[test]
fn download_torrent_saves_file() {
    let server = MockServer::start();
    let download_path = "/download.php/111222/file.torrent";
    let _m = server.mock(|when, then| {
        when.method(GET).path(download_path);
        then.status(200)
            .header(
                "content-disposition",
                "attachment; filename=\"custom-name.torrent\"",
            )
            .body(FAKE_TORRENT_BYTES);
    });

    let client = test_client(server.base_url());
    let tmp = tempdir().expect("temp dir");
    let out = download_torrent(
        &client,
        &format!("{}{}", server.base_url(), download_path),
        tmp.path(),
        None,
    )
    .expect("download should work");

    assert!(out.exists());
    assert_eq!(
        out.file_name().and_then(|n| n.to_str()),
        Some("custom-name.torrent")
    );
    assert_eq!(std::fs::read(out).expect("read output"), FAKE_TORRENT_BYTES);
}

#[test]
fn active_returns_seeding_and_leeching() {
    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET).path("/peers");
        then.status(200).body(PEERS_HTML);
    });

    let client = test_client(server.base_url());
    let results = active(&client).expect("active should succeed");

    assert_eq!(results.seeding.len(), 2);
    assert_eq!(results.leeching.len(), 1);
}
