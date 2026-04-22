pub mod cli;
pub mod config;
pub mod download;
pub mod error;
pub mod http;
pub mod info;
pub mod models;
pub mod output;
pub mod search;
pub mod utils;

#[cfg(test)]
#[path = "../tests/support/fixtures.rs"]
pub(crate) mod test_fixtures;
