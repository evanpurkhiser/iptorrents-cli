"""CLI entry point for iptorrents-cli."""

import argparse
import json
import sys
from pathlib import Path
from typing import Any

import requests
from toon_format import encode as toon_encode

from .config import AUTH_FILE, get_auth_cookies, get_config, write_auth
from .download import download_torrent, stream_torrent
from .info import fetch_info
from .search import SORT_FIELDS, search
from .session import make_session, verify_session


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="ipt",
        description="Search and download torrents from IPTorrents.",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Output as JSON instead of TOON.",
    )

    sub = parser.add_subparsers(dest="command", metavar="COMMAND")

    # --- auth ---
    p_auth = sub.add_parser(
        "auth",
        help="Save IPTorrents cookies for authentication.",
        description=(
            "Parse and save cookies from a browser cookie string.\n\n"
            "Copy the cookie string from DevTools (Network tab → any iptorrents.com "
            "request → Request Headers → Cookie) and pass it here.\n\n"
            "Or use the JavaScript snippet in extras/get-cookies.js to copy it "
            "to your clipboard automatically."
        ),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    p_auth.add_argument(
        "cookies",
        help=(
            'Cookie header string, e.g. "uid=123; pass=abc; cf_clearance=xyz". '
            'Reads from stdin if value is "-".'
        ),
    )

    # --- search ---
    p_search = sub.add_parser("search", aliases=["s"], help="Search for torrents.")
    p_search.add_argument("query", nargs="+", help="Search terms.")
    p_search.add_argument(
        "-s",
        "--sort",
        metavar="FIELD",
        help=f"Sort by: {', '.join(SORT_FIELDS)}",
    )
    p_search.add_argument(
        "-n",
        "--limit",
        type=int,
        default=25,
        metavar="N",
        help="Max results (default: 25).",
    )

    # --- info ---
    p_info = sub.add_parser("info", aliases=["i"], help="Show details for a torrent by ID.")
    p_info.add_argument("id", type=int, help="Torrent ID.")

    # --- download ---
    p_dl = sub.add_parser("download", aliases=["d"], help="Download a .torrent file by ID.")
    p_dl.add_argument("id", type=int, help="Torrent ID.")
    p_dl.add_argument(
        "-o",
        "--output",
        metavar="DIR",
        help="Directory to save the .torrent file (default: current directory).",
    )
    p_dl.add_argument(
        "--stdout",
        action="store_true",
        help="Write raw .torrent bytes to stdout instead of saving to disk.",
    )

    return parser


def get_session() -> requests.Session:
    config = get_config()
    uid, pass_cookie, cf_clearance = get_auth_cookies(config)
    session = make_session(uid, pass_cookie, cf_clearance)
    verify_session(session)
    return session


def _output(data: Any, use_json: bool) -> None:
    """Serialize data as TOON (default) or JSON."""
    if use_json:
        print(json.dumps(data, indent=2))
    else:
        print(toon_encode(data))


def cmd_auth(args: argparse.Namespace) -> None:
    """Parse a browser cookie string and save credentials to the auth file."""
    raw = sys.stdin.read().strip() if args.cookies == "-" else args.cookies

    # Parse "key=value; key=value; ..." — tolerant of extra whitespace
    cookies: dict[str, str] = {}
    for part in raw.split(";"):
        part = part.strip()
        if "=" in part:
            k, _, v = part.partition("=")
            cookies[k.strip()] = v.strip()

    uid = cookies.get("uid", "")
    pass_cookie = cookies.get("pass", "")
    cf_clearance = cookies.get("cf_clearance") or None

    if not uid or not pass_cookie:
        missing = [k for k, v in [("uid", uid), ("pass", pass_cookie)] if not v]
        print(
            f"Error: cookie string is missing required key(s): {', '.join(missing)}",
            file=sys.stderr,
        )
        print(
            "Expected format:  uid=<value>; pass=<value>[; cf_clearance=<value>]", file=sys.stderr
        )
        sys.exit(1)

    write_auth(uid, pass_cookie, cf_clearance)
    print(f"Credentials saved to {AUTH_FILE}")
    if cf_clearance:
        print("  uid, pass, cf_clearance stored.")
    else:
        print("  uid, pass stored (no cf_clearance — add it if requests are blocked).")


def cmd_search(args: argparse.Namespace, use_json: bool) -> None:
    session = get_session()
    query = " ".join(args.query)
    results = search(session, query, limit=args.limit, sort=args.sort)

    _output(
        [
            {
                "id": t.id,
                "name": t.name,
                "category": t.category,
                "size": t.size,
                "seeders": t.seeders,
                "leechers": t.leechers,
                "downloads": t.downloads,
                "added": t.added,
                "freeleech": t.freeleech,
                "download_url": t.download_url,
            }
            for t in results
        ],
        use_json,
    )


def cmd_info(args: argparse.Namespace, use_json: bool) -> None:
    session = get_session()
    info = fetch_info(session, args.id)
    _output(
        {
            "id": info.id,
            "name": info.name,
            "size": info.size,
            "file_count": info.file_count,
            "uploaded": info.uploaded,
            "uploader": info.uploader,
            "seeders": info.seeders,
            "leechers": info.leechers,
            "genre": info.genre,
            "plot": info.plot,
            "actors": info.actors,
            "imdb_url": info.imdb_url,
            "tmdb_url": info.tmdb_url,
            "download_url": info.download_url,
        },
        use_json,
    )


def cmd_download(args: argparse.Namespace, use_json: bool) -> None:
    session = get_session()

    if args.stdout:
        # Stream raw bytes to stdout — only torrent bytes must reach stdout.
        # Skip fetch_info entirely; construct the download URL directly from the ID
        # so nothing else can accidentally pollute stdout.
        dl_url = f"https://iptorrents.com/download.php/{args.id}/{args.id}.torrent"
        stream_torrent(session, dl_url, sys.stdout.buffer)
        return

    info = fetch_info(session, args.id)
    dest = Path(args.output) if args.output else Path.cwd()
    out_path = download_torrent(session, info.download_url, dest_dir=dest)
    _output({"path": str(out_path), "id": args.id}, use_json)


def main() -> None:
    parser = build_parser()
    args = parser.parse_args()
    use_json: bool = args.json

    match args.command:
        case "auth":
            cmd_auth(args)
        case "search" | "s":
            cmd_search(args, use_json)
        case "info" | "i":
            cmd_info(args, use_json)
        case "download" | "d":
            cmd_download(args, use_json)
        case _:
            parser.print_help()


if __name__ == "__main__":
    main()
