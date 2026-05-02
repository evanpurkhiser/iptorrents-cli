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

pub const PEERS_HTML: &str = r#"<!DOCTYPE html><html><body>
<table class=t1>
  <tr>
    <th>Torrent</th><th>User</th><th>%</th><th>Uploaded</th><th>Rate</th>
    <th>Downloaded</th><th>Rate</th><th>Seeding Time</th><th>User Agent</th><th>IP Address</th>
  </tr>
  <tr><td colspan=99 class=ac>Seeders</td></tr>
  <tr>
    <td><a href="?t=6788723">28 Years Later 2025 HDR 2160p WEB h265-ETHEL</a></td>
    <td><a href="?u=1283939"></a></td>
    <td>100%</td>
    <td class=ar>329 MB (16.1 GB)</td>
    <td class=ar>5.3 MB/s</td>
    <td class=ar>0 B (11.8 GB)</td>
    <td class=ar>0 B/s</td>
    <td class=ar>7.7 months</td>
    <td>Transmission/4.1.1</td>
    <td><a href="?ipa=74.64.39.221">74.64.39.221</a></td>
  </tr>
  <tr>
    <td><a href="?t=886881">A Beautiful Mind 2001 1080p BluRay x264-KaKa</a></td>
    <td><a href="?u=1283939"></a></td>
    <td>100%</td>
    <td class=ar>0 B (18.4 GB)</td>
    <td class=ar>0 B/s</td>
    <td class=ar>0 B (10.9 GB)</td>
    <td class=ar>0 B/s</td>
    <td class=ar>8.1 months</td>
    <td>Transmission/4.1.1</td>
    <td><a href="?ipa=74.64.39.221">74.64.39.221</a></td>
  </tr>
  <tr><td colspan=99 class=ac>Leechers</td></tr>
  <tr>
    <td><a href="?t=7200316">A Beautiful Mind 2001 UHD BluRay 2160p DDP Atmo...</a></td>
    <td><a href="?u=1283939"></a></td>
    <td>54%</td>
    <td class=ar>3.93 GB (7.38 GB)</td>
    <td class=ar>120 KB/s</td>
    <td class=ar>0 B (18 GB)</td>
    <td class=ar>1.2 MB/s</td>
    <td class=ar>3.1 weeks</td>
    <td>Transmission/4.1.1</td>
    <td><a href="?ipa=74.64.39.221">74.64.39.221</a></td>
  </tr>
</table>
</body></html>"#;
