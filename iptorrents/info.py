"""Fetch and parse individual torrent detail pages from IPTorrents."""

import re
import sys
from dataclasses import dataclass

import requests
from bs4 import BeautifulSoup

from .session import BASE_URL
from .utils import parse_int


@dataclass(frozen=True, slots=True)
class TorrentInfo:
    id: int
    name: str
    size: str
    uploaded: str
    uploader: str
    seeders: int
    leechers: int
    file_count: int
    genre: tuple[str, ...]
    plot: str
    actors: tuple[str, ...]
    imdb_url: str
    tmdb_url: str
    download_url: str

    def __str__(self) -> str:
        lines = [
            f"[{self.id}] {self.name}",
            f"  Size     : {self.size} in {self.file_count} file(s)",
            f"  Uploaded : {self.uploaded}  by {self.uploader}",
            f"  Seeders  : {self.seeders}  Leechers: {self.leechers}",
        ]
        if self.genre:
            lines.append(f"  Genre    : {', '.join(self.genre)}")
        if self.plot:
            lines.append(f"  Plot     : {self.plot}")
        if self.actors:
            lines.append(f"  Actors   : {', '.join(self.actors)}")
        if self.imdb_url:
            lines.append(f"  IMDb     : {self.imdb_url}")
        if self.tmdb_url:
            lines.append(f"  TMDB     : {self.tmdb_url}")
        return "\n".join(lines)


def fetch_info(session: requests.Session, torrent_id: int) -> TorrentInfo:
    """Fetch and parse the detail page for a torrent."""
    url = f"{BASE_URL}/t/{torrent_id}"
    try:
        r = session.get(url, timeout=15)
        r.raise_for_status()
    except (requests.RequestException, OSError) as e:
        print(f"Error: could not fetch torrent page: {e}", file=sys.stderr)
        sys.exit(1)

    return _parse_detail(r.text, torrent_id)


def _parse_detail(html: str, torrent_id: int) -> TorrentInfo:
    soup = BeautifulSoup(html, "lxml")
    for tag in soup(["script", "style"]):
        tag.decompose()

    tables = soup.find_all("table")

    # --- name from page title ---
    name = ""
    title_tag = soup.find("title")
    if title_tag:
        # "Blade Runner 2049 - IPTorrents - #1 Private Tracker"
        name = title_tag.get_text(strip=True).split(" - IPTorrents")[0].strip()

    # --- stats table (table[1]): seeders, leechers, size, uploader, upload date ---
    size = uploaded = uploader = ""
    seeders = leechers = file_count = 0
    if len(tables) > 1:
        stats = tables[1]
        txt = stats.get_text(" ", strip=True)

        size_m = re.search(r"Size:\s*([\d.,]+\s*[KMGT]?B)\s*in\s*(\d+)\s*file", txt)
        if size_m:
            size = size_m.group(1)
            file_count = int(size_m.group(2))

        up_tag = stats.find("span", class_="elapsedDate")
        if up_tag:
            uploaded = up_tag.get_text(strip=True)

        uploader_links = stats.find_all("a", href=re.compile(r"/u/\d+"))
        for ul in uploader_links:
            txt = ul.get_text(strip=True)
            if txt:
                uploader = txt
                break

        peer_link = stats.find("a", class_="peer")
        if peer_link:
            spans = peer_link.find_all("span")
            if len(spans) >= 2:
                seeders = parse_int(spans[0].get_text())
                leechers = parse_int(spans[1].get_text())

    # --- description table (table[2]): genre, plot, actors, external links ---
    genre: list[str] = []
    plot = ""
    actors: list[str] = []
    imdb_url = tmdb_url = ""
    if len(tables) > 2:
        desc = tables[2]
        for row in desc.find_all("tr"):
            cells = row.find_all("td")
            if len(cells) < 2:
                continue
            label = cells[0].get_text(strip=True).lower()
            value_cell = cells[1]

            match label:
                case "genre":
                    genre = [a.get_text(strip=True) for a in value_cell.find_all("a")]
                case "plot":
                    plot = value_cell.get_text(strip=True)
                case "actors":
                    actors = [a.get_text(strip=True) for a in value_cell.find_all("a")]
                case _:
                    # External links row (IMDb, TMDB)
                    for a in value_cell.find_all("a", href=True):
                        href = a["href"]
                        if "imdb.com" in href:
                            imdb_url = href
                        elif "themoviedb.org" in href:
                            tmdb_url = href

    # --- download URL ---
    dl_link = soup.find("a", href=re.compile(r"download\.php"))
    if dl_link:
        href = dl_link["href"]
        dl_url = href if href.startswith("http") else f"{BASE_URL}/{href.lstrip('/')}"
    else:
        dl_url = f"{BASE_URL}/download.php/{torrent_id}/{torrent_id}.torrent"

    return TorrentInfo(
        id=torrent_id,
        name=name,
        size=size,
        uploaded=uploaded,
        uploader=uploader,
        seeders=seeders,
        leechers=leechers,
        file_count=file_count,
        genre=tuple(genre),
        plot=plot,
        actors=tuple(actors),
        imdb_url=imdb_url,
        tmdb_url=tmdb_url,
        download_url=dl_url,
    )
