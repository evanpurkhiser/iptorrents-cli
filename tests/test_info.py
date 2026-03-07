"""Tests for iptorrents.info — detail page parsing across categories."""

import pytest

from iptorrents.info import _parse_detail

from .fixtures import INFO_HTML_MOVIE, INFO_HTML_MUSIC, INFO_HTML_SOFTWARE


@pytest.fixture
def movie_info():
    return _parse_detail(INFO_HTML_MOVIE, torrent_id=111222)


@pytest.fixture
def software_info():
    return _parse_detail(INFO_HTML_SOFTWARE, torrent_id=333444)


@pytest.fixture
def music_info():
    return _parse_detail(INFO_HTML_MUSIC, torrent_id=555666)


class TestParseDetailMovie:
    def test_id(self, movie_info):
        assert movie_info.id == 111222

    def test_name_stripped_of_site_suffix(self, movie_info):
        assert movie_info.name == "Blade Runner 2049 2017 2160p UHD BluRay x265"

    def test_size(self, movie_info):
        assert movie_info.size == "55.3 GB"

    def test_file_count(self, movie_info):
        assert movie_info.file_count == 3

    def test_uploader(self, movie_info):
        assert movie_info.uploader == "uploader_guy"

    def test_uploaded_date(self, movie_info):
        assert movie_info.uploaded == "3 days ago"

    def test_seeders_leechers(self, movie_info):
        assert movie_info.seeders == 987
        assert movie_info.leechers == 12

    def test_genre(self, movie_info):
        assert movie_info.genre == ("Sci-Fi", "Drama")

    def test_plot(self, movie_info):
        assert "blade runner" in movie_info.plot.lower()

    def test_actors(self, movie_info):
        assert "Ryan Gosling" in movie_info.actors
        assert "Harrison Ford" in movie_info.actors

    def test_imdb_url(self, movie_info):
        assert "imdb.com" in movie_info.imdb_url

    def test_tmdb_url(self, movie_info):
        assert "themoviedb.org" in movie_info.tmdb_url

    def test_download_url_absolute(self, movie_info):
        assert movie_info.download_url.startswith("https://")
        assert "111222" in movie_info.download_url


class TestParseDetailSoftware:
    """Software torrents have no table[2] — parser must not crash."""

    def test_id(self, software_info):
        assert software_info.id == 333444

    def test_name(self, software_info):
        assert "Ubuntu" in software_info.name

    def test_size(self, software_info):
        assert software_info.size == "5.68 GB"

    def test_file_count(self, software_info):
        assert software_info.file_count == 1

    def test_uploader(self, software_info):
        assert software_info.uploader == "linuxfan"

    def test_seeders_leechers(self, software_info):
        assert software_info.seeders == 503
        assert software_info.leechers == 78

    def test_genre_empty(self, software_info):
        assert software_info.genre == ()

    def test_plot_empty(self, software_info):
        assert software_info.plot == ""

    def test_actors_empty(self, software_info):
        assert software_info.actors == ()

    def test_imdb_tmdb_empty(self, software_info):
        assert software_info.imdb_url == ""
        assert software_info.tmdb_url == ""

    def test_download_url(self, software_info):
        assert software_info.download_url.startswith("https://")
        assert "333444" in software_info.download_url


class TestParseDetailMusic:
    """Music torrents have table[2] but none of the expected fields."""

    def test_name(self, music_info):
        assert "Pink Floyd" in music_info.name

    def test_size(self, music_info):
        assert music_info.size == "1.2 GB"

    def test_file_count(self, music_info):
        assert music_info.file_count == 24

    def test_uploader(self, music_info):
        assert music_info.uploader == "audiophile99"

    def test_seeders(self, music_info):
        assert music_info.seeders == 210

    def test_genre_empty_when_row_absent(self, music_info):
        assert music_info.genre == ()

    def test_plot_empty_when_row_absent(self, music_info):
        assert music_info.plot == ""

    def test_actors_empty_when_row_absent(self, music_info):
        assert music_info.actors == ()

    def test_external_links_empty(self, music_info):
        assert music_info.imdb_url == ""
        assert music_info.tmdb_url == ""

    def test_download_url(self, music_info):
        assert "555666" in music_info.download_url
