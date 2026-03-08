"""Search IPTorrents and parse results."""

import re
import sys
from dataclasses import dataclass

import requests
from bs4 import BeautifulSoup

from .session import BASE_URL
from .utils import parse_int

# Maps user-facing sort names to IPTorrents URL parameter values.
# Note: "name" cannot be a StrEnum member (conflicts with Enum.name descriptor),
# so this stays a plain dict.
SORT_FIELDS: dict[str, str] = {
    "seeders": "seeders",
    "leechers": "leechers",
    "size": "size",
    "downloads": "completed",  # IPTorrents internal name
    "name": "name",
    "age": "age",
}


@dataclass(frozen=True, slots=True)
class Torrent:
    id: int
    name: str
    category: str
    size: str
    seeders: int
    leechers: int
    downloads: int
    added: str
    download_url: str
    freeleech: bool = False

    def __str__(self) -> str:
        fl = " [FL]" if self.freeleech else ""
        return (
            f"[{self.id}] {self.name}{fl}\n"
            f"  Category : {self.category}\n"
            f"  Size     : {self.size}  |  "
            f"Seeders: {self.seeders}  Leechers: {self.leechers}  Downloads: {self.downloads}\n"
            f"  Added    : {self.added}"
        )


def search(
    session: requests.Session,
    query: str,
    limit: int = 25,
    sort: str | None = None,
) -> list[Torrent]:
    """Search IPTorrents and return a list of Torrent results.

    Args:
        session: authenticated requests.Session
        query: search terms
        limit: max results to return
        sort: sort field — one of: seeders, leechers, size, downloads, name, age
    """
    params: dict[str, str | int] = {"q": query, "qf": ""}
    if sort:
        o = SORT_FIELDS.get(sort)
        if o is None:
            print(
                f"Error: unknown sort field '{sort}'. Valid options: {', '.join(SORT_FIELDS)}",
                file=sys.stderr,
            )
            sys.exit(1)
        params["o"] = o

    try:
        r = session.get(f"{BASE_URL}/t", params=params, timeout=20)
        r.raise_for_status()
    except (requests.RequestException, OSError) as e:
        print(f"Error: search request failed: {e}", file=sys.stderr)
        sys.exit(1)

    return _parse_results(r.text, limit)


def _parse_results(html: str, limit: int) -> list[Torrent]:
    soup = BeautifulSoup(html, "lxml")

    table = soup.find("table", id="torrents")
    if table is None:
        if "sign in" in html.lower():
            print(
                "Error: not logged in — run `ipt auth` to save credentials.",
                file=sys.stderr,
            )
            sys.exit(1)
        return []

    results: list[Torrent] = []
    rows = table.find_all("tr")[1:]  # skip header

    for row in rows[:limit]:
        cols = row.find_all("td")
        if len(cols) < 9:
            continue
        try:
            # col 0: category img
            # col 1: name + metadata
            # col 2: bookmark
            # col 3: download link
            # col 4: comments
            # col 5: size
            # col 6: snatches/downloads
            # col 7: seeders
            # col 8: leechers

            cat_img = cols[0].find("img")
            category = cat_img["alt"].strip() if cat_img and cat_img.get("alt") else "Unknown"

            name_link = cols[1].find("a", href=re.compile(r"/t/\d+"))
            if not name_link:
                continue
            name = name_link.get_text(strip=True)
            torrent_id_match = re.search(r"/t/(\d+)", name_link["href"])
            if not torrent_id_match:
                continue
            torrent_id = int(torrent_id_match.group(1))

            age_match = re.search(r"(\d+ \w+ ago)", cols[1].get_text(" ", strip=True))
            added = age_match.group(1) if age_match else ""

            # Freeleech: IPTorrents marks FL torrents with a tag in the name cell.
            # Matches <b class="fl">, <span class="freeleech">, or any element whose
            # text is "FreeLeech" / "Free Leech" (case-insensitive).
            fl_tag = cols[1].find(
                lambda tag: (
                    any("fl" in c.lower() or "freeleech" in c.lower() for c in tag.get("class", []))
                    or re.search(r"free\s*leech", tag.get_text(), re.IGNORECASE) is not None
                )
            )
            freeleech = fl_tag is not None

            dl_link = cols[3].find("a", href=re.compile(r"download\.php"))
            if dl_link:
                href = dl_link["href"]
                dl_url = href if href.startswith("http") else f"{BASE_URL}/{href.lstrip('/')}"
            else:
                dl_url = f"{BASE_URL}/download.php/{torrent_id}/{torrent_id}.torrent"

            size = cols[5].get_text(strip=True)
            downloads = parse_int(cols[6].get_text(strip=True))
            seeders = parse_int(cols[7].get_text(strip=True))
            leechers = parse_int(cols[8].get_text(strip=True))

            results.append(
                Torrent(
                    id=torrent_id,
                    name=name,
                    category=category,
                    size=size,
                    seeders=seeders,
                    leechers=leechers,
                    downloads=downloads,
                    added=added,
                    download_url=dl_url,
                    freeleech=freeleech,
                )
            )
        except (IndexError, KeyError, TypeError, AttributeError):
            continue

    return results
