"""Tests for iptorrents.search — HTML parsing logic."""

from iptorrents.search import _parse_results
from iptorrents.utils import parse_int as _parse_int

from .fixtures import SEARCH_HTML, SEARCH_HTML_EMPTY


class TestParseResults:
    def test_returns_all_rows(self):
        results = _parse_results(SEARCH_HTML, limit=25)
        assert len(results) == 3

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

    # --- freeleech ---

    def test_non_freeleech_rows_are_false(self):
        results = _parse_results(SEARCH_HTML, limit=25)
        assert results[0].freeleech is False  # Blade Runner
        assert results[1].freeleech is False  # Ubuntu

    def test_freeleech_detected_via_fl_class(self):
        results = _parse_results(SEARCH_HTML, limit=25)
        tv = results[2]  # The Bear — has <b class="fl">
        assert tv.freeleech is True
        assert tv.id == 555777

    def test_freeleech_via_text(self):
        """A <span> with text 'FreeLeech' should also be detected."""
        html = """\
<!DOCTYPE html><html><body>
<table id="torrents">
  <tr><th>Cat</th><th>Name</th><th>Bkm</th><th>DL</th>
      <th>Cmt</th><th>Size</th><th>Snatches</th><th>Seeders</th><th>Leechers</th></tr>
  <tr>
    <td><img alt="Movie/HD" /></td>
    <td>
      <a href="/t/999">Some Movie 1080p</a>
      <span>Free Leech</span>
      <span>1 day ago</span>
    </td>
    <td></td>
    <td><a href="download.php/999/Some.Movie.torrent">DL</a></td>
    <td>0</td><td>8 GB</td><td>100</td><td>50</td><td>5</td>
  </tr>
</table>
</body></html>"""
        results = _parse_results(html, limit=25)
        assert len(results) == 1
        assert results[0].freeleech is True

    def test_freeleech_via_freeleech_class(self):
        """A tag with class containing 'freeleech' should be detected."""
        html = """\
<!DOCTYPE html><html><body>
<table id="torrents">
  <tr><th>Cat</th><th>Name</th><th>Bkm</th><th>DL</th>
      <th>Cmt</th><th>Size</th><th>Snatches</th><th>Seeders</th><th>Leechers</th></tr>
  <tr>
    <td><img alt="TV/HD" /></td>
    <td>
      <a href="/t/888">Show S01E01 720p</a>
      <span class="freeleech">FL</span>
      <span>5 hours ago</span>
    </td>
    <td></td>
    <td><a href="download.php/888/Show.torrent">DL</a></td>
    <td>0</td><td>2 GB</td><td>20</td><td>300</td><td>10</td>
  </tr>
</table>
</body></html>"""
        results = _parse_results(html, limit=25)
        assert len(results) == 1
        assert results[0].freeleech is True


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
