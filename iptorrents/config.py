"""Configuration management for iptorrents-cli.

Auth state location: ~/.local/state/iptorrents-cli/auth.toml

Example auth file (written by `ipt auth` or by hand):

    [auth]
    uid = "123456"
    pass = "abc123def456..."
    cf_clearance = "abc..."   # optional but usually required
"""

import sys
import tomllib
from pathlib import Path
from typing import Any

STATE_DIR = Path.home() / ".local" / "state" / "iptorrents-cli"
AUTH_FILE = STATE_DIR / "auth.toml"


def ensure_state_dir() -> None:
    """Create the state directory with restrictive permissions (700)."""
    STATE_DIR.mkdir(parents=True, exist_ok=True)
    STATE_DIR.chmod(0o700)


def _check_file_permissions(path: Path) -> None:
    """Warn if the auth file is readable by group or others."""
    mode = path.stat().st_mode & 0o777
    if mode & 0o077:
        print(
            f"Warning: {path} has permissions {oct(mode)} — "
            "run `chmod 600` to restrict access to your cookies.",
            file=sys.stderr,
        )


def get_config() -> dict[str, Any]:
    """Load and return the auth file as a dict. Exits with an error if missing."""
    if not AUTH_FILE.exists():
        print(f"Error: auth file not found at {AUTH_FILE}", file=sys.stderr)
        print("", file=sys.stderr)
        print("Run:  ipt auth <cookie-string>", file=sys.stderr)
        print("Or create it manually:", file=sys.stderr)
        print("", file=sys.stderr)
        print(f"  mkdir -p {STATE_DIR}", file=sys.stderr)
        print(f"  cat > {AUTH_FILE} << 'EOF'", file=sys.stderr)
        print("  [auth]", file=sys.stderr)
        print('  uid = "123456"', file=sys.stderr)
        print('  pass = "abc123..."', file=sys.stderr)
        print('  cf_clearance = "abc..."', file=sys.stderr)
        print("  EOF", file=sys.stderr)
        print(f"  chmod 600 {AUTH_FILE}", file=sys.stderr)
        sys.exit(1)

    try:
        _check_file_permissions(AUTH_FILE)
        with AUTH_FILE.open("rb") as f:
            return tomllib.load(f)
    except PermissionError as e:
        print(f"Error: cannot read auth file: {e}", file=sys.stderr)
        sys.exit(1)
    except tomllib.TOMLDecodeError as e:
        print(f"Error: auth file is not valid TOML: {e}", file=sys.stderr)
        sys.exit(1)


def get_auth_cookies(config: dict[str, Any]) -> tuple[str, str, str | None]:
    """Extract uid, pass, and optional cf_clearance cookie values from config."""
    try:
        auth = config["auth"]
        uid = auth["uid"]
        pass_cookie = auth["pass"]
    except KeyError as e:
        key = e.args[0]
        print(f"Error: missing config key: [auth].{key}", file=sys.stderr)
        print("Run `ipt auth <cookie-string>` to set credentials.", file=sys.stderr)
        sys.exit(1)

    if not uid or not pass_cookie:
        print("Error: [auth].uid and [auth].pass must not be empty.", file=sys.stderr)
        sys.exit(1)

    return uid, pass_cookie, auth.get("cf_clearance") or None


def write_auth(uid: str, pass_cookie: str, cf_clearance: str | None) -> None:
    """Write (or overwrite) the auth file with the given cookie values."""
    ensure_state_dir()
    lines = [
        "[auth]\n",
        f'uid = "{uid}"\n',
        f'pass = "{pass_cookie}"\n',
    ]
    if cf_clearance:
        lines.append(f'cf_clearance = "{cf_clearance}"\n')
    AUTH_FILE.write_text("".join(lines))
    AUTH_FILE.chmod(0o600)
