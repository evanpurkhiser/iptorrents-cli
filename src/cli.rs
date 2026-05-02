use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "ipt")]
#[command(about = "Search and download torrents from IPTorrents.")]
pub struct Cli {
    #[arg(long, global = true, help = "Output as JSON instead of TOON.")]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Save IPTorrents cookies for authentication.")]
    Auth {
        #[arg(
            help = "Cookie header string, e.g. \"uid=123; pass=abc; cf_clearance=xyz\". Reads from stdin if value is '-'."
        )]
        cookies: String,
    },

    #[command(alias = "s", about = "Search for torrents.")]
    Search {
        #[arg(required = true, help = "Search terms.")]
        query: Vec<String>,

        #[arg(
            short = 's',
            long = "sort",
            help = "Sort by: seeders, leechers, size, downloads, name, age"
        )]
        sort: Option<String>,

        #[arg(
            short = 'n',
            long = "limit",
            default_value_t = 25,
            help = "Max results (default: 25)."
        )]
        limit: usize,
    },

    #[command(alias = "i", about = "Show details for a torrent by ID.")]
    Info {
        #[arg(help = "Torrent ID.")]
        id: i64,
    },

    #[command(alias = "d", about = "Download a .torrent file by ID.")]
    Download {
        #[arg(help = "Torrent ID.")]
        id: i64,

        #[arg(
            short = 'o',
            long = "output",
            value_name = "DIR",
            help = "Directory to save the .torrent file (default: current directory)."
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "stdout",
            help = "Write raw .torrent bytes to stdout instead of saving to disk."
        )]
        stdout: bool,
    },

    #[command(alias = "a", about = "Show active seeding and leeching torrents.")]
    Active,
}
