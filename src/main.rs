use std::path::PathBuf;

use clap::Parser;

use iptorrents_cli::active::active;
use iptorrents_cli::cli::{Cli, Commands};
use iptorrents_cli::config::{self, read_auth_config, write_auth_file};
use iptorrents_cli::download::{download_torrent, stream_torrent};
use iptorrents_cli::error::Result;
use iptorrents_cli::http::{self, IptClient, verify_session};
use iptorrents_cli::info::fetch_info;
use iptorrents_cli::output::{OutputFormat, emit};
use iptorrents_cli::search::search;

fn authed_client() -> Result<IptClient> {
    let auth = read_auth_config()?;
    let client = IptClient::new(
        auth,
        std::env::var("IPT_BASE_URL").unwrap_or_else(|_| http::BASE_URL.to_string()),
    )?;
    verify_session(&client)?;
    Ok(client)
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let format = if cli.json {
        OutputFormat::Json
    } else {
        OutputFormat::Toon
    };

    match cli.command {
        Commands::Auth { cookies } => {
            let auth = config::parse_cookie_string_arg(&cookies)?;
            let path = write_auth_file(&auth)?;
            println!("Credentials saved to {}", path.display());
            if auth.cf_clearance.is_some() {
                println!("  uid, pass, cf_clearance stored.");
            } else {
                println!("  uid, pass stored (no cf_clearance - add it if requests are blocked).");
            }
        }
        Commands::Search { query, sort, limit } => {
            let client = authed_client()?;
            let query = query.join(" ");
            let results = search(&client, &query, limit, sort.as_deref())?;
            emit(&results, format)?;
        }
        Commands::Info { id } => {
            let client = authed_client()?;
            let info = fetch_info(&client, id)?;
            emit(&info, format)?;
        }
        Commands::Download { id, output, stdout } => {
            let client = authed_client()?;

            if stdout {
                let download_url = format!("{}/download.php/{id}/{id}.torrent", client.base_url());
                let mut out = std::io::stdout().lock();
                stream_torrent(&client, &download_url, &mut out)?;
                return Ok(());
            }

            let info = fetch_info(&client, id)?;
            let dest_dir = output.unwrap_or_else(|| PathBuf::from("."));
            let path = download_torrent(&client, &info.download_url, &dest_dir, None)?;
            emit(
                &serde_json::json!({
                    "path": path.display().to_string(),
                    "id": id
                }),
                format,
            )?;
        }
        Commands::Active => {
            let client = authed_client()?;
            let results = active(&client)?;
            emit(&results, format)?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
