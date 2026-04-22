#![allow(dead_code)]

pub const SEARCH_HTML: &str = r#"<!DOCTYPE html><html><body>
<table id="torrents">
  <tr>
    <th>Cat</th><th>Name</th><th>Bkm</th><th>DL</th>
    <th>Cmt</th><th>Size</th><th>Snatches</th><th>Seeders</th><th>Leechers</th>
  </tr>
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
  <tr>
    <td><img alt="TV/HD" /></td>
    <td>
      <a href="/t/555777">The Bear S03E05 1080p WEB-DL x264</a>
      <b class="fl">FreeLeech</b>
      <span>2 hours ago</span>
    </td>
    <td></td>
    <td><a href="download.php/555777/The.Bear.S03E05.torrent">DL</a></td>
    <td>3</td>
    <td>3.2 GB</td>
    <td>456</td>
    <td>1200</td>
    <td>55</td>
  </tr>
</table>
</body></html>"#;

pub const SEARCH_HTML_LOGGED_OUT: &str =
    r#"<!DOCTYPE html><html><body><h1>Please sign in to continue</h1></body></html>"#;

pub const SEARCH_HTML_EMPTY: &str = r#"<!DOCTYPE html><html><body>
<table id="torrents">
  <tr><th>Cat</th><th>Name</th><th>Bkm</th><th>DL</th>
      <th>Cmt</th><th>Size</th><th>Snatches</th><th>Seeders</th><th>Leechers</th></tr>
</table>
</body></html>"#;

pub const INFO_HTML_MOVIE: &str = r#"<!DOCTYPE html><html><head>
<title>Blade Runner 2049 2017 2160p UHD BluRay x265 - IPTorrents - #1 Private Tracker</title>
</head><body>
<table><tr><td>nav</td></tr></table>
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
</body></html>"#;

pub const INFO_HTML_SOFTWARE: &str = r#"<!DOCTYPE html><html><head>
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
<a href="download.php/333444/ubuntu-24.04.torrent">Download</a>
</body></html>"#;

pub const INFO_HTML_MUSIC: &str = r#"<!DOCTYPE html><html><head>
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
<table>
  <tr><td>Format</td><td>FLAC</td></tr>
  <tr><td>Bitrate</td><td>Lossless</td></tr>
</table>
<a href="download.php/555666/Pink.Floyd.Wall.torrent">Download</a>
</body></html>"#;
