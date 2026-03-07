"""HTTP session management for IPTorrents — cookie-based auth."""

import sys

import requests

BASE_URL = "https://iptorrents.com"

# Cloudflare expects a real browser UA when cf_clearance is present
_USER_AGENT = "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0"


def make_session(uid: str, pass_cookie: str, cf_clearance: str | None = None) -> requests.Session:
    """Create an authenticated requests.Session using IPTorrents browser cookies.

    Args:
        uid: value of the 'uid' cookie from iptorrents.com
        pass_cookie: value of the 'pass' cookie from iptorrents.com
        cf_clearance: optional Cloudflare clearance cookie (required if CF is active)
    """
    s = requests.Session()
    s.headers.update(
        {
            "User-Agent": _USER_AGENT,
            "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            "Accept-Language": "en-US,en;q=0.5",
        }
    )
    s.cookies.set("uid", uid, domain="iptorrents.com")
    s.cookies.set("pass", pass_cookie, domain="iptorrents.com")
    if cf_clearance:
        s.cookies.set("cf_clearance", cf_clearance, domain="iptorrents.com")
    return s


def verify_session(session: requests.Session) -> None:
    """Check that the session is valid; exit with an error if not."""
    try:
        r = session.get(f"{BASE_URL}/t", allow_redirects=False, timeout=15)
    except (requests.RequestException, OSError) as e:
        print(f"Error: could not reach IPTorrents: {e}", file=sys.stderr)
        sys.exit(1)
    if r.status_code != 200:
        print("Error: IPTorrents session is invalid or expired.", file=sys.stderr)
        print(
            "Update the uid, pass, and cf_clearance values in ~/.config/iptorrents/config.toml",
            file=sys.stderr,
        )
        print("(copy fresh cookies from your browser's DevTools)", file=sys.stderr)
        sys.exit(1)
