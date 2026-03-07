"""Download .torrent files from IPTorrents."""

import re
import sys
from pathlib import Path
from typing import BinaryIO
from urllib.parse import unquote

import requests

from .utils import safe_filename


def _fetch(session: requests.Session, download_url: str) -> requests.Response:
    """Fetch download_url, exit on error."""
    try:
        r = session.get(download_url, timeout=30, stream=True)
        r.raise_for_status()
    except (requests.RequestException, OSError) as e:
        print(f"Error: download failed: {e}", file=sys.stderr)
        sys.exit(1)
    return r


def _resolve_filename(r: requests.Response, download_url: str, filename: str | None) -> str:
    """Determine a safe local filename from an override, Content-Disposition, or URL."""
    resolved: str
    if filename:
        resolved = filename
    else:
        cd = r.headers.get("Content-Disposition", "")
        # RFC 6266 — prefer quoted form, fall back to unquoted, then URL basename
        m = re.search(r'filename="([^"]+)"', cd) or re.search(r"filename=([^\s;]+)", cd)
        resolved = m.group(1) if m else unquote(download_url.rstrip("/").split("/")[-1])

    # Strip directory components to prevent path traversal
    resolved = safe_filename(resolved)

    if not resolved.endswith(".torrent"):
        resolved = resolved + ".torrent"
    return resolved


def download_torrent(
    session: requests.Session,
    download_url: str,
    dest_dir: Path | None = None,
    filename: str | None = None,
) -> Path:
    """Download a .torrent file and save it to disk.

    Args:
        session: authenticated requests.Session
        download_url: direct .torrent download URL
        dest_dir: directory to save into (defaults to current directory)
        filename: override filename (defaults to Content-Disposition or URL basename)

    Returns:
        Path to the saved .torrent file.
    """
    dest_dir = dest_dir or Path.cwd()
    dest_dir.mkdir(parents=True, exist_ok=True)

    r = _fetch(session, download_url)
    filename = _resolve_filename(r, download_url, filename)

    out_path = dest_dir / filename
    with out_path.open("wb") as f:
        for chunk in r.iter_content(chunk_size=8192):
            f.write(chunk)

    return out_path


def stream_torrent(
    session: requests.Session,
    download_url: str,
    dest: BinaryIO,
) -> None:
    """Stream raw .torrent bytes directly to *dest* (e.g. sys.stdout.buffer).

    Nothing is written to disk.  No filename resolution is needed.

    Args:
        session: authenticated requests.Session
        download_url: direct .torrent download URL
        dest: binary file-like object to write to
    """
    r = _fetch(session, download_url)
    for chunk in r.iter_content(chunk_size=8192):
        dest.write(chunk)
