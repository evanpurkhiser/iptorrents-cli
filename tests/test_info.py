"""Tests for iptorrents.info — detail page parsing across categories."""

from iptorrents.info import _parse_detail

from .fixtures import INFO_HTML_MOVIE, INFO_HTML_MUSIC, INFO_HTML_SOFTWARE


class TestParseDetailMovie:
    def setup_method(self):
        self.info = _parse_detail(INFO_HTML_MOVIE, torrent_id=111222)

    def test_id(self):
        assert self.info.id == 111222

    def test_name_stripped_of_site_suffix(self):
        assert self.info.name == "Blade Runner 2049 2017 2160p UHD BluRay x265"

    def test_size(self):
        assert self.info.size == "55.3 GB"

    def test_file_count(self):
        assert self.info.file_count == 3

    def test_uploader(self):
        assert self.info.uploader == "uploader_guy"

    def test_uploaded_date(self):
        assert self.info.uploaded == "3 days ago"

    def test_seeders_leechers(self):
        assert self.info.seeders == 987
        assert self.info.leechers == 12

    def test_genre(self):
        assert self.info.genre == ["Sci-Fi", "Drama"]

    def test_plot(self):
        assert "blade runner" in self.info.plot.lower()

    def test_actors(self):
        assert "Ryan Gosling" in self.info.actors
        assert "Harrison Ford" in self.info.actors

    def test_imdb_url(self):
        assert "imdb.com" in self.info.imdb_url

    def test_tmdb_url(self):
        assert "themoviedb.org" in self.info.tmdb_url

    def test_download_url_absolute(self):
        assert self.info.download_url.startswith("https://")
        assert "111222" in self.info.download_url


class TestParseDetailSoftware:
    """Software torrents have no table[2] — parser must not crash."""

    def setup_method(self):
        self.info = _parse_detail(INFO_HTML_SOFTWARE, torrent_id=333444)

    def test_id(self):
        assert self.info.id == 333444

    def test_name(self):
        assert "Ubuntu" in self.info.name

    def test_size(self):
        assert self.info.size == "5.68 GB"

    def test_file_count(self):
        assert self.info.file_count == 1

    def test_uploader(self):
        assert self.info.uploader == "linuxfan"

    def test_seeders_leechers(self):
        assert self.info.seeders == 503
        assert self.info.leechers == 78

    def test_genre_empty(self):
        assert self.info.genre == []

    def test_plot_empty(self):
        assert self.info.plot == ""

    def test_actors_empty(self):
        assert self.info.actors == []

    def test_imdb_tmdb_empty(self):
        assert self.info.imdb_url == ""
        assert self.info.tmdb_url == ""

    def test_download_url(self):
        assert self.info.download_url.startswith("https://")
        assert "333444" in self.info.download_url


class TestParseDetailMusic:
    """Music torrents have table[2] but none of the expected fields."""

    def setup_method(self):
        self.info = _parse_detail(INFO_HTML_MUSIC, torrent_id=555666)

    def test_name(self):
        assert "Pink Floyd" in self.info.name

    def test_size(self):
        assert self.info.size == "1.2 GB"

    def test_file_count(self):
        assert self.info.file_count == 24

    def test_uploader(self):
        assert self.info.uploader == "audiophile99"

    def test_seeders(self):
        assert self.info.seeders == 210

    def test_genre_empty_when_row_absent(self):
        assert self.info.genre == []

    def test_plot_empty_when_row_absent(self):
        assert self.info.plot == ""

    def test_actors_empty_when_row_absent(self):
        assert self.info.actors == []

    def test_external_links_empty(self):
        assert self.info.imdb_url == ""
        assert self.info.tmdb_url == ""

    def test_download_url(self):
        assert "555666" in self.info.download_url
