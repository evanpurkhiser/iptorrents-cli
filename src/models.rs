use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct AuthFile {
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub uid: String,
    #[serde(rename = "pass")]
    pub pass_cookie: String,
    pub cf_clearance: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Torrent {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub size: String,
    pub seeders: i64,
    pub leechers: i64,
    pub downloads: i64,
    pub added: String,
    pub freeleech: bool,
    pub download_url: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TorrentInfo {
    pub id: i64,
    pub name: String,
    pub size: String,
    pub uploaded: String,
    pub uploader: String,
    pub seeders: i64,
    pub leechers: i64,
    pub file_count: i64,
    pub genre: Vec<String>,
    pub plot: String,
    pub actors: Vec<String>,
    pub imdb_url: String,
    pub tmdb_url: String,
    pub download_url: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ActiveTorrent {
    #[serde(rename = "torrent")]
    pub name: String,
    #[serde(rename = "torrent_id")]
    pub id: i64,
    pub percent: String,
    pub uploaded: String,
    pub upload_rate: String,
    pub downloaded: String,
    pub download_rate: String,
    pub seeding_time: String,
    pub user_agent: String,
    pub ip_address: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ActiveTorrents {
    pub seeding: Vec<ActiveTorrent>,
    pub leeching: Vec<ActiveTorrent>,
}
