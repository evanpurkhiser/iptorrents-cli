"""HTML fixtures that mirror the real IPTorrents page structure."""

# ---------------------------------------------------------------------------
# Search results page — /t?q=...
# Two rows: one movie, one software (no movie metadata)
# ---------------------------------------------------------------------------
SEARCH_HTML = """\
<!DOCTYPE html><html><body>
<table id="torrents">
  <tr>
    <th>Cat</th><th>Name</th><th>Bkm</th><th>DL</th>
    <th>Cmt</th><th>Size</th><th>Snatches</th><th>Seeders</th><th>Leechers</th>
  </tr>
  <!-- movie row -->
  <tr>
    <td><img alt="Movie/HD" /></td>
    <td>
      <a href="/t/111222">Blade Runner 2049 2017 2160p UHD BluRay x265</a>
      <span>3 days ago</span>
    </td>
    <td></td>
    <td><a href="download.php/111222/Blade.Runner.torrent">DL</a></td>
    <td>0</td>
    <td>55.3 GB</td>
    <td>1,234</td>
    <td>987</td>
    <td>12</td>
  </tr>
  <!-- software / linux iso row -->
  <tr>
    <td><img alt="PC/0day" /></td>
    <td>
      <a href="/t/333444">Ubuntu 24.04 LTS Desktop amd64</a>
      <span>1 week ago</span>
    </td>
    <td></td>
    <td><a href="download.php/333444/ubuntu-24.04.torrent">DL</a></td>
    <td>0</td>
    <td>5.68 GB</td>
    <td>9,999</td>
    <td>503</td>
    <td>78</td>
  </tr>
</table>
</body></html>
"""

# Search results where the page contains "sign in" (expired session)
SEARCH_HTML_LOGGED_OUT = """\
<!DOCTYPE html><html><body>
<h1>Please sign in to continue</h1>
</body></html>
"""

# Search results with an empty table (no matches)
SEARCH_HTML_EMPTY = """\
<!DOCTYPE html><html><body>
<table id="torrents">
  <tr><th>Cat</th><th>Name</th><th>Bkm</th><th>DL</th>
      <th>Cmt</th><th>Size</th><th>Snatches</th><th>Seeders</th><th>Leechers</th></tr>
</table>
</body></html>
"""

# ---------------------------------------------------------------------------
# Torrent detail page — /t/111222  (movie with full metadata)
# ---------------------------------------------------------------------------
INFO_HTML_MOVIE = """\
<!DOCTYPE html><html><head>
<title>Blade Runner 2049 2017 2160p UHD BluRay x265 - IPTorrents - #1 Private Tracker</title>
</head><body>
<!-- table[0]: navigation / breadcrumb (ignored) -->
<table><tr><td>nav</td></tr></table>

<!-- table[1]: stats -->
<table>
  <tr>
    <td>Size: 55.3 GB in 3 files</td>
    <td>
      Uploaded by <a href="/u/0"></a> <a href="/u/42">uploader_guy</a>
      <span class="elapsedDate">3 days ago</span>
    </td>
    <td>
      <a class="peer">
        <span>987</span><span>12</span>
      </a>
    </td>
  </tr>
</table>

<!-- table[2]: description with full movie metadata -->
<table>
  <tr><td>Genre</td><td><a>Sci-Fi</a> <a>Drama</a></td></tr>
  <tr><td>Plot</td><td>A young blade runner uncovers a secret.</td></tr>
  <tr><td>Actors</td><td><a>Ryan Gosling</a> <a>Harrison Ford</a></td></tr>
  <tr>
    <td>Links</td>
    <td>
      <a href="https://www.imdb.com/title/tt1856101/">IMDb</a>
      <a href="https://www.themoviedb.org/movie/335984">TMDB</a>
    </td>
  </tr>
</table>

<a href="download.php/111222/Blade.Runner.2049.torrent">Download</a>
</body></html>
"""

# ---------------------------------------------------------------------------
# Torrent detail page — /t/333444  (software — no table[2] at all)
# ---------------------------------------------------------------------------
INFO_HTML_SOFTWARE = """\
<!DOCTYPE html><html><head>
<title>Ubuntu 24.04 LTS Desktop amd64 - IPTorrents - #1 Private Tracker</title>
</head><body>
<table><tr><td>nav</td></tr></table>

<table>
  <tr>
    <td>Size: 5.68 GB in 1 files</td>
    <td>
      Uploaded by <a href="/u/0"></a> <a href="/u/7">linuxfan</a>
      <span class="elapsedDate">1 week ago</span>
    </td>
    <td>
      <a class="peer">
        <span>503</span><span>78</span>
      </a>
    </td>
  </tr>
</table>

<!-- NO table[2] for software — parser must not crash -->

<a href="download.php/333444/ubuntu-24.04.torrent">Download</a>
</body></html>
"""

# ---------------------------------------------------------------------------
# Torrent detail page — /t/555666  (music — table[2] present but no genre/plot/actors)
# ---------------------------------------------------------------------------
INFO_HTML_MUSIC = """\
<!DOCTYPE html><html><head>
<title>Pink Floyd - The Wall FLAC - IPTorrents - #1 Private Tracker</title>
</head><body>
<table><tr><td>nav</td></tr></table>

<table>
  <tr>
    <td>Size: 1.2 GB in 24 files</td>
    <td>
      Uploaded by <a href="/u/0"></a> <a href="/u/99">audiophile99</a>
      <span class="elapsedDate">5 days ago</span>
    </td>
    <td>
      <a class="peer">
        <span>210</span><span>5</span>
      </a>
    </td>
  </tr>
</table>

<!-- table[2] exists but has no genre / plot / actors / external links -->
<table>
  <tr><td>Format</td><td>FLAC</td></tr>
  <tr><td>Bitrate</td><td>Lossless</td></tr>
</table>

<a href="download.php/555666/Pink.Floyd.Wall.torrent">Download</a>
</body></html>
"""

# ---------------------------------------------------------------------------
# Verify session response — just needs to be a 200 with any content
# ---------------------------------------------------------------------------
SESSION_OK_HTML = "<html><body>ok</body></html>"
"""Minimal 200 response that satisfies verify_session."""
