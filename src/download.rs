use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use crate::error::Result;
use crate::http::IptClient;
use crate::utils::safe_filename;

static QUOTED_FILENAME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"filename=\"([^\"]+)\""#).expect("valid regex"));
static UNQUOTED_FILENAME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"filename=([^\s;]+)").expect("valid regex"));

fn fetch(client: &IptClient, download_url: &str) -> Result<reqwest::blocking::Response> {
    let response = if let Some(path) = download_url.strip_prefix(client.base_url()) {
        client.get(path).send()?
    } else if download_url.starts_with("http://") || download_url.starts_with("https://") {
        client.get_absolute(download_url).send()?
    } else {
        client
            .get(&format!("/{}", download_url.trim_start_matches('/')))
            .send()?
    };
    Ok(response.error_for_status()?)
}

fn resolve_filename(
    response: &reqwest::blocking::Response,
    download_url: &str,
    filename: Option<&str>,
) -> String {
    let mut resolved = if let Some(filename) = filename {
        filename.to_string()
    } else {
        let cd = response
            .headers()
            .get("content-disposition")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if let Some(caps) = QUOTED_FILENAME_RE.captures(cd) {
            caps.get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default()
        } else if let Some(caps) = UNQUOTED_FILENAME_RE.captures(cd) {
            caps.get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default()
        } else {
            download_url
                .split('/')
                .next_back()
                .map(ToString::to_string)
                .unwrap_or_else(|| "download.torrent".to_string())
        }
    };

    resolved = safe_filename(&resolved);
    if !resolved.ends_with(".torrent") {
        resolved.push_str(".torrent");
    }
    resolved
}

pub fn download_torrent(
    client: &IptClient,
    download_url: &str,
    dest_dir: &Path,
    filename: Option<&str>,
) -> Result<PathBuf> {
    fs::create_dir_all(dest_dir)?;
    let mut response = fetch(client, download_url)?;
    let name = resolve_filename(&response, download_url, filename);
    let out_path = dest_dir.join(name);

    let mut file = File::create(&out_path)?;
    io::copy(&mut response, &mut file)?;

    Ok(out_path)
}

pub fn stream_torrent(client: &IptClient, download_url: &str, dest: &mut dyn Write) -> Result<()> {
    let mut response = fetch(client, download_url)?;
    io::copy(&mut response, dest)?;
    Ok(())
}
