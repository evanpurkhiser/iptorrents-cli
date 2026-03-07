"""IPTorrents category IDs.

These are the numeric category IDs used in IPTorrents search URLs.
e.g. /t?q=query&c72=1  means "search in Movies/x265".
"""

# fmt: off
CATEGORIES: dict[str, int] = {
    # Movies
    "movies"            : 0,    # all movie categories (meta)
    "movies/x265-4k"   : 320,
    "movies/x265-4k-hdr": 2145,
    "movies/x265-4k-web": 2148,
    "movies/bluray"     : 2,
    "movies/bluray-4k"  : 1401,
    "movies/dvd"        : 47,
    "movies/dvd-sd"     : 16,
    "movies/dvd-extras" : 399,
    "movies/xvid"       : 48,
    "movies/x265"       : 285,
    "movies/web-dl"     : 2135,
    "movies/x265-hd"    : 720,
    "movies/other"      : 50,

    # TV
    "tv"                : 0,    # meta
    "tv/x265-4k"        : 2144,
    "tv/x265-4k-web"    : 2149,
    "tv/bluray"         : 201,
    "tv/dvd"            : 26,
    "tv/dvd-sd"         : 55,
    "tv/xvid"           : 54,
    "tv/web-dl"         : 2137,
    "tv/web-dl-x265"    : 2139,
    "tv/x265-hd"        : 2255,
    "tv/other"          : 29,

    # Games
    "games/pc"          : 2,
    "games/mac"         : 54,
    "games/ps3"         : 6,
    "games/ps4"         : 34,
    "games/psp"         : 55,
    "games/xbox360"     : 14,
    "games/xboxone"     : 35,
    "games/wii"         : 9,
    "games/wiiu"        : 45,
    "games/3ds"         : 46,
    "games/switch"      : 2195,

    # Music
    "music/mp3"         : 31,
    "music/flac"        : 32,
    "music/video"       : 52,
    "music/other"       : 53,

    # Apps
    "apps/windows"      : 33,
    "apps/mac"          : 38,
    "apps/linux"        : 37,
    "apps/android"      : 39,
    "apps/ios"          : 40,
    "apps/other"        : 36,

    # Books
    "books/ebooks"      : 65,
    "books/audiobooks"  : 67,
    "books/comics"      : 66,
    "books/magazines"   : 68,

    # Anime
    "anime"             : 60,

    # Adult
    "xxx"               : 2161,
    "xxx/video"         : 2162,
    "xxx/other"         : 2163,

    # Misc
    "other"             : 64,
}
# fmt: on


def resolve_category(name: str) -> int | None:
    """Return numeric category ID for a name, or None if not found."""
    return CATEGORIES.get(name.lower().replace(" ", "/"))


def list_categories() -> list[str]:
    return sorted(CATEGORIES.keys())
