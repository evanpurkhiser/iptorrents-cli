# iptorrents-cli

A focused CLI for searching and downloading torrents from [IPTorrents](https://iptorrents.com).
Designed for agent/LLM use — output defaults to [TOON format](https://github.com/toon-format/toon-python) (token-efficient), with `--json` as an override.

---

## Installation

```sh
git clone https://github.com/youruser/iptorrents-cli
cd iptorrents-cli
uv tool install .
```

---

## Authentication

IPTorrents uses Cloudflare Turnstile — automated login is impossible. Auth is done via browser cookies.

### 1. Extract cookies

Open [iptorrents.com](https://iptorrents.com), log in, open DevTools (F12) → Application/Storage → Cookies, and copy the values for:
- `uid`
- `pass`
- `cf_clearance` (Cloudflare token)

Format them as: `uid=VALUE; pass=VALUE; cf_clearance=VALUE`

### 2. Save credentials

```sh
ipt auth "uid=123456; pass=abc123...; cf_clearance=xyz..."
```

Credentials are saved to `~/.local/state/iptorrents-cli/auth.toml` with `chmod 600`.

---

## Commands

```
ipt [--json] COMMAND ...
```

`--json` switches all output to pretty-printed JSON. Default is TOON.

---

### `ipt auth`

Save browser cookies for authentication.

```sh
ipt auth "uid=123456; pass=abc...; cf_clearance=xyz..."

# Read from stdin:
echo "uid=123456; pass=abc..." | ipt auth -
```

---

### `ipt search` (`ipt s`)

Search for torrents.

```sh
ipt search "blade runner 2049"
ipt s "ubuntu 24.04" --sort seeders --limit 10
ipt search "pink floyd" --json
```

| Flag | Default | Description |
|---|---|---|
| `-s / --sort FIELD` | age | `seeders` `leechers` `size` `downloads` `name` `age` |
| `-n / --limit N` | 25 | Max results |

Output fields: `id`, `name`, `category`, `size`, `seeders`, `leechers`, `downloads`, `added`, `download_url`

---

### `ipt info` (`ipt i`)

Show full details for a torrent by ID.

```sh
ipt info 111222
ipt i 111222 --json
```

Output fields: `id`, `name`, `size`, `file_count`, `uploaded`, `uploader`, `seeders`, `leechers`, `genre`, `plot`, `actors`, `imdb_url`, `tmdb_url`, `download_url`

Movie/TV torrents include `genre`, `plot`, `actors`, `imdb_url`, `tmdb_url`. Other categories leave these empty.

---

### `ipt download` (`ipt d`)

Download a `.torrent` file by ID.

```sh
# Save to current directory:
ipt download 111222

# Save to a specific directory:
ipt d 111222 --output ~/Downloads

# Stream raw bytes to stdout — pipe directly to transmission-cli:
ipt download --stdout 111222 | transmission-remote --add -
```

| Flag | Description |
|---|---|
| `-o / --output DIR` | Directory to save `.torrent` (default: cwd) |
| `--stdout` | Write raw bytes to stdout, suppress all other output |

---

## Output formats

### TOON (default)

Compact tabular notation — minimal tokens, great for LLM consumption:

```
[id name category size seeders leechers downloads added download_url]
[111222 "Blade Runner 2049 2017 2160p UHD BluRay x265" "Movie/HD" "55.3 GB" 987 12 1234 "3 days ago" "https://iptorrents.com/download.php/111222/..."]
[333444 "Ubuntu 24.04 LTS Desktop amd64" "PC/0day" "5.68 GB" 503 78 9999 "1 week ago" "https://iptorrents.com/download.php/333444/..."]
```

### JSON

```sh
ipt search "hevc remux" --json | jq '.[].seeders'
```

---

## Auth file

`~/.local/state/iptorrents-cli/auth.toml` (created by `ipt auth`, `chmod 600`):

```toml
[auth]
uid = "123456"
pass = "abc123def456..."
cf_clearance = "xyz..."   # optional but usually needed
```

---

## Development

```sh
uv sync
uv run pytest tests/ -v
uv run ruff check iptorrents/
```

---

## Security

- Cookies grant full account access. The auth file is stored `600`, the directory `700`.
- Never commit `~/.local/state/iptorrents-cli/auth.toml`.
- Enabling `requests` debug logging will print cookie values to stderr.
