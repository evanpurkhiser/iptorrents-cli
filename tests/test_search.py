"""Tests for iptorrents.search — HTML parsing logic."""

from iptorrents.search import _parse_results
from iptorrents.utils import parse_int as _parse_int

from .fixtures import SEARCH_HTML, SEARCH_HTML_EMPTY


class TestParseResults:
    def test_returns_both_rows(self):
        results = _parse_results(SEARCH_HTML, limit=25)
        assert len(results) == 2

    def test_movie_row_fields(self):
        results = _parse_results(SEARCH_HTML, limit=25)
        movie = results[0]
        assert movie.id == 111222
        assert "Blade Runner" in movie.name
        assert movie.category == "Movie/HD"
        assert movie.size == "55.3 GB"
        assert movie.seeders == 987
        assert movie.leechers == 12
        assert movie.downloads == 1234
        assert "3 days ago" in movie.added
        assert (
            movie.download_url == "https://iptorrents.com/download.php/111222/Blade.Runner.torrent"
        )

    def test_software_row_fields(self):
        results = _parse_results(SEARCH_HTML, limit=25)
        sw = results[1]
        assert sw.id == 333444
        assert "Ubuntu" in sw.name
        assert sw.category == "PC/0day"
        assert sw.seeders == 503
        assert sw.leechers == 78
        assert sw.downloads == 9999

    def test_limit_is_respected(self):
        results = _parse_results(SEARCH_HTML, limit=1)
        assert len(results) == 1
        assert results[0].id == 111222

    def test_empty_table_returns_empty_list(self):
        results = _parse_results(SEARCH_HTML_EMPTY, limit=25)
        assert results == []

    def test_download_url_always_absolute(self):
        """Relative download.php links must be made absolute."""
        results = _parse_results(SEARCH_HTML, limit=25)
        for t in results:
            assert t.download_url.startswith("https://")

    def test_no_torrents_table_returns_empty_list(self):
        html = "<html><body><p>nothing here</p></body></html>"
        results = _parse_results(html, limit=25)
        assert results == []


class TestParseInt:
    def test_plain_number(self):
        assert _parse_int("987") == 987

    def test_comma_separated(self):
        assert _parse_int("1,234") == 1234

    def test_empty_string(self):
        assert _parse_int("") == 0

    def test_whitespace(self):
        assert _parse_int("  42  ") == 42

    def test_non_numeric(self):
        assert _parse_int("n/a") == 0
