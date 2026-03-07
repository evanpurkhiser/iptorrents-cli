"""Tests for HTTP-layer functions (search, fetch_info, download_torrent).

Uses the `responses` library to intercept requests.Session calls so no
real network traffic is made.
"""

from __future__ import annotations

from pathlib import Path

import pytest
import responses as rsps_lib

import io

from iptorrents.download import download_torrent, stream_torrent
from iptorrents.info import fetch_info
from iptorrents.search import search
from iptorrents.session import BASE_URL, make_session

from .fixtures import (
    INFO_HTML_MOVIE,
    INFO_HTML_SOFTWARE,
    SEARCH_HTML,
    SEARCH_HTML_LOGGED_OUT,
    SESSION_OK_HTML,
)


@pytest.fixture()
def session():
    """Unauthenticated-but-valid session (cookies are fake — responses lib intercepts)."""
    return make_session(uid="fake_uid", pass_cookie="fake_pass", cf_clearance="fake_cf")


# ---------------------------------------------------------------------------
# search()
# ---------------------------------------------------------------------------


class TestSearch:
    @rsps_lib.activate
    def test_returns_results(self, session):
        rsps_lib.add(rsps_lib.GET, f"{BASE_URL}/t", body=SEARCH_HTML, status=200)
        results = search(session, "blade runner")
        assert len(results) == 2

    @rsps_lib.activate
    def test_movie_result_fields(self, session):
        rsps_lib.add(rsps_lib.GET, f"{BASE_URL}/t", body=SEARCH_HTML, status=200)
        results = search(session, "blade runner")
        movie = results[0]
        assert movie.id == 111222
        assert movie.category == "Movie/HD"
        assert movie.seeders == 987

    @rsps_lib.activate
    def test_limit_is_forwarded(self, session):
        rsps_lib.add(rsps_lib.GET, f"{BASE_URL}/t", body=SEARCH_HTML, status=200)
        results = search(session, "blade runner", limit=1)
        assert len(results) == 1

    @rsps_lib.activate
    def test_sort_param_sent(self, session):
        rsps_lib.add(rsps_lib.GET, f"{BASE_URL}/t", body=SEARCH_HTML, status=200)
        search(session, "ubuntu", sort="seeders")
        assert len(rsps_lib.calls) == 1
        assert "o=seeders" in rsps_lib.calls[0].request.url

    @rsps_lib.activate
    def test_network_error_exits(self, session):
        rsps_lib.add(rsps_lib.GET, f"{BASE_URL}/t", body=ConnectionError("offline"))
        with pytest.raises(SystemExit):
            search(session, "anything")

    @rsps_lib.activate
    def test_logged_out_page_exits(self, session):
        rsps_lib.add(rsps_lib.GET, f"{BASE_URL}/t", body=SEARCH_HTML_LOGGED_OUT, status=200)
        with pytest.raises(SystemExit):
            search(session, "anything")


# ---------------------------------------------------------------------------
# fetch_info()
# ---------------------------------------------------------------------------


class TestFetchInfo:
    @rsps_lib.activate
    def test_movie_parsed_correctly(self, session):
        rsps_lib.add(rsps_lib.GET, f"{BASE_URL}/t/111222", body=INFO_HTML_MOVIE, status=200)
        info = fetch_info(session, 111222)
        assert info.id == 111222
        assert "Blade Runner" in info.name
        assert info.seeders == 987
        assert info.genre == ("Sci-Fi", "Drama")
        assert "imdb.com" in info.imdb_url

    @rsps_lib.activate
    def test_software_no_crash(self, session):
        """fetch_info must not crash for torrents without movie metadata."""
        rsps_lib.add(rsps_lib.GET, f"{BASE_URL}/t/333444", body=INFO_HTML_SOFTWARE, status=200)
        info = fetch_info(session, 333444)
        assert info.id == 333444
        assert info.genre == ()
        assert info.plot == ""
        assert info.imdb_url == ""

    @rsps_lib.activate
    def test_http_error_exits(self, session):
        rsps_lib.add(rsps_lib.GET, f"{BASE_URL}/t/99999", status=403)
        with pytest.raises(SystemExit):
            fetch_info(session, 99999)

    @rsps_lib.activate
    def test_network_error_exits(self, session):
        rsps_lib.add(rsps_lib.GET, f"{BASE_URL}/t/99999", body=ConnectionError("offline"))
        with pytest.raises(SystemExit):
            fetch_info(session, 99999)


# ---------------------------------------------------------------------------
# download_torrent()
# ---------------------------------------------------------------------------


FAKE_TORRENT_BYTES = b"d8:announce35:http://tracker.example.com/announcee"


class TestStreamTorrent:
    @rsps_lib.activate
    def test_writes_bytes_to_stream(self, session):
        dl_url = f"{BASE_URL}/download.php/111222/file.torrent"
        rsps_lib.add(rsps_lib.GET, dl_url, body=FAKE_TORRENT_BYTES, status=200)
        buf = io.BytesIO()
        stream_torrent(session, dl_url, buf)
        assert buf.getvalue() == FAKE_TORRENT_BYTES

    @rsps_lib.activate
    def test_nothing_written_on_http_error(self, session):
        dl_url = f"{BASE_URL}/download.php/0/bad.torrent"
        rsps_lib.add(rsps_lib.GET, dl_url, status=403)
        buf = io.BytesIO()
        with pytest.raises(SystemExit):
            stream_torrent(session, dl_url, buf)
        assert buf.getvalue() == b""

    @rsps_lib.activate
    def test_no_file_created_on_disk(self, session, tmp_path):
        dl_url = f"{BASE_URL}/download.php/111222/file.torrent"
        rsps_lib.add(rsps_lib.GET, dl_url, body=FAKE_TORRENT_BYTES, status=200)
        buf = io.BytesIO()
        stream_torrent(session, dl_url, buf)
        assert list(tmp_path.iterdir()) == []


class TestDownloadTorrent:
    @rsps_lib.activate
    def test_saves_file_to_disk(self, session, tmp_path):
        dl_url = f"{BASE_URL}/download.php/111222/Blade.Runner.2049.torrent"
        rsps_lib.add(
            rsps_lib.GET,
            dl_url,
            body=FAKE_TORRENT_BYTES,
            status=200,
            headers={"Content-Disposition": 'attachment; filename="Blade.Runner.2049.torrent"'},
            content_type="application/x-bittorrent",
        )
        out = download_torrent(session, dl_url, dest_dir=tmp_path)
        assert out.exists()
        assert out.read_bytes() == FAKE_TORRENT_BYTES
        assert out.suffix == ".torrent"

    @rsps_lib.activate
    def test_filename_from_content_disposition(self, session, tmp_path):
        dl_url = f"{BASE_URL}/download.php/111222/file.torrent"
        rsps_lib.add(
            rsps_lib.GET,
            dl_url,
            body=FAKE_TORRENT_BYTES,
            status=200,
            headers={"Content-Disposition": 'attachment; filename="custom-name.torrent"'},
        )
        out = download_torrent(session, dl_url, dest_dir=tmp_path)
        assert out.name == "custom-name.torrent"

    @rsps_lib.activate
    def test_filename_falls_back_to_url(self, session, tmp_path):
        dl_url = f"{BASE_URL}/download.php/111222/fallback-name.torrent"
        rsps_lib.add(rsps_lib.GET, dl_url, body=FAKE_TORRENT_BYTES, status=200)
        out = download_torrent(session, dl_url, dest_dir=tmp_path)
        assert out.name == "fallback-name.torrent"

    @rsps_lib.activate
    def test_explicit_filename_override(self, session, tmp_path):
        dl_url = f"{BASE_URL}/download.php/111222/whatever.torrent"
        rsps_lib.add(rsps_lib.GET, dl_url, body=FAKE_TORRENT_BYTES, status=200)
        out = download_torrent(session, dl_url, dest_dir=tmp_path, filename="my-override.torrent")
        assert out.name == "my-override.torrent"

    @rsps_lib.activate
    def test_torrent_extension_appended_if_missing(self, session, tmp_path):
        dl_url = f"{BASE_URL}/download.php/111222/noext"
        rsps_lib.add(
            rsps_lib.GET,
            dl_url,
            body=FAKE_TORRENT_BYTES,
            status=200,
        )
        out = download_torrent(session, dl_url, dest_dir=tmp_path, filename="noext")
        assert out.name == "noext.torrent"

    @rsps_lib.activate
    def test_http_error_exits(self, session, tmp_path):
        dl_url = f"{BASE_URL}/download.php/0/bad.torrent"
        rsps_lib.add(rsps_lib.GET, dl_url, status=404)
        with pytest.raises(SystemExit):
            download_torrent(session, dl_url, dest_dir=tmp_path)

    @rsps_lib.activate
    def test_dest_dir_created_if_missing(self, session, tmp_path):
        dl_url = f"{BASE_URL}/download.php/111222/file.torrent"
        rsps_lib.add(rsps_lib.GET, dl_url, body=FAKE_TORRENT_BYTES, status=200)
        new_dir = tmp_path / "deep" / "nested"
        assert not new_dir.exists()
        out = download_torrent(session, dl_url, dest_dir=new_dir)
        assert new_dir.exists()
        assert out.exists()
