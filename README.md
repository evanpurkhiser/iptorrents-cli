# iptorrents-cli

A focused Rust CLI for searching and downloading torrents from [IPTorrents](https://iptorrents.com).

Output defaults to TOON (token-efficient). Use `--json` for pretty JSON.

## Installation

```sh
# From source
git clone https://github.com/evanpurkhiser/iptorrents-cli
cd iptorrents-cli
cargo install --path .

# Or run directly in the repo
cargo run -- search "ubuntu"
```

## Authentication

IPTorrents uses Cloudflare Turnstile. Automated login is not supported.
Use browser cookies from an authenticated session.

1. Open <https://iptorrents.com> in your browser.
2. Copy cookie values for:
   - `uid`
   - `pass`
   - `cf_clearance` (usually required)
3. Save them:

```sh
ipt auth "uid=123456; pass=abc123...; cf_clearance=xyz..."
```

Credentials are stored at:

- `~/.local/state/iptorrents-cli/auth.toml`
- state dir permissions: `700`
- auth file permissions: `600`

## Commands

```sh
ipt [--json] COMMAND ...
```

### `ipt auth`

Save browser cookies.

```sh
ipt auth "uid=123456; pass=abc...; cf_clearance=xyz..."

# Or read from stdin
echo "uid=123456; pass=abc..." | ipt auth -
```

### `ipt search` (`ipt s`)

Search for torrents.

```sh
ipt search "blade runner 2049"
ipt s "ubuntu 24.04" --sort seeders --limit 10
ipt --json search "pink floyd"
```

Flags:

- `-s, --sort` one of: `seeders`, `leechers`, `size`, `downloads`, `name`, `age`
- `-n, --limit` max results (default: `25`)

Output fields:

- `id`, `name`, `category`, `size`, `seeders`, `leechers`, `downloads`, `added`, `freeleech`, `download_url`

### `ipt info` (`ipt i`)

Show details for a torrent by ID.

```sh
ipt info 111222
ipt --json i 111222
```

Output fields:

- `id`, `name`, `size`, `file_count`, `uploaded`, `uploader`, `seeders`, `leechers`
- `genre`, `plot`, `actors`, `imdb_url`, `tmdb_url`, `download_url`

### `ipt download` (`ipt d`)

Download a `.torrent` file by ID.

```sh
# Save in current directory
ipt download 111222

# Save in a specific directory
ipt d 111222 --output ~/Downloads

# Stream raw bytes to stdout
ipt download --stdout 111222 | transmission-remote --add -
```

Flags:

- `-o, --output DIR` output directory (default: current directory)
- `--stdout` stream raw bytes to stdout instead of saving to disk

### `ipt active` (`ipt a`)

Show your currently active torrents from the peers page, grouped by seeding and
leeching.

```sh
ipt active
ipt --json active
```

Output shape:

- top-level keys: `seeding`, `leeching`
- each key contains a list of objects with:
  `torrent`, `torrent_id`, `percent`, `uploaded`, `upload_rate`, `downloaded`,
  `download_rate`, `seeding_time`, `user_agent`, `ip_address`

## Output formats

### TOON (default)

Compact token-efficient format via `serde_toon`.

### JSON

```sh
ipt --json search "hevc remux" | jq '.[].seeders'
```

## Development

```sh
cargo fmt
cargo test
```

## Security

- Cookies grant full account access.
- Never commit `~/.local/state/iptorrents-cli/auth.toml`.
- Be careful with debug logs that may expose cookie values.
