"""Shared utilities for iptorrents-cli."""

from __future__ import annotations

import re
from pathlib import Path


def parse_int(s: str) -> int:
    """Parse a possibly comma-separated integer string, returning 0 on failure."""
    cleaned = re.sub(r"[^\d]", "", s)
    return int(cleaned) if cleaned else 0


def safe_filename(name: str) -> str:
    """Strip any directory components from a server-supplied filename.

    Prevents path-traversal attacks where Content-Disposition returns
    something like ``../../.bashrc``.
    """
    return Path(name).name
