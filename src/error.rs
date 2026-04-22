use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("auth file not found at {0}")]
    MissingAuthFile(PathBuf),

    #[error("missing config key: [auth].{0}")]
    MissingAuthKey(&'static str),

    #[error("[auth].uid and [auth].pass must not be empty")]
    EmptyAuthValues,

    #[error("cookie string is missing required key(s): {0}")]
    MissingCookieKeys(String),

    #[error("unknown sort field '{0}'. Valid options: {1}")]
    InvalidSortField(String, String),

    #[error("not logged in - run `ipt auth` to save credentials")]
    NotLoggedIn,

    #[error("IPTorrents session is invalid or expired")]
    InvalidSession,

    #[error("could not determine parent directory for auth file")]
    MissingAuthParentDir,

    #[error("invalid cookie header: {0}")]
    InvalidCookieHeader(String),

    #[error("unexpected search page layout")]
    UnexpectedSearchLayout,

    #[error("could not parse torrent details page (unexpected HTML layout)")]
    UnexpectedDetailLayout,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Toon(#[from] serde_toon::Error),
}
