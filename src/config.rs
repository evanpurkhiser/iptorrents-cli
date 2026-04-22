use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use cookie::Cookie;
use serde::Serialize;
use xdg::BaseDirectories;

use crate::error::{Error, Result};
use crate::models::{AuthConfig, AuthFile};

#[derive(Debug, Serialize)]
struct AuthWrite<'a> {
    auth: AuthWriteSection<'a>,
}

#[derive(Debug, Serialize)]
struct AuthWriteSection<'a> {
    uid: &'a str,
    #[serde(rename = "pass")]
    pass_cookie: &'a str,
    cf_clearance: &'a Option<String>,
}

pub fn auth_file_path() -> Result<PathBuf> {
    let xdg = BaseDirectories::with_prefix("iptorrents-cli");

    if let Some(existing) = xdg.find_state_file("auth.toml") {
        return Ok(existing);
    }

    xdg.place_state_file("auth.toml")
        .map_err(|e| Error::XdgStatePath(e.to_string()))
}

fn state_dir_path() -> Result<PathBuf> {
    let path = auth_file_path()?;
    path.parent()
        .map(PathBuf::from)
        .ok_or(Error::MissingAuthParentDir)
}

fn ensure_state_dir_permissions() -> Result<()> {
    let dir = state_dir_path()?;
    fs::create_dir_all(&dir)?;
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
    Ok(())
}

pub fn read_auth_config() -> Result<AuthConfig> {
    let path = auth_file_path()?;
    if !path.exists() {
        return Err(Error::MissingAuthFile(path));
    }

    let meta = fs::metadata(&path)?;
    let mode = meta.permissions().mode() & 0o777;
    if mode & 0o077 != 0 {
        eprintln!(
            "Warning: {} has permissions {:o} - run `chmod 600` to restrict access to your cookies.",
            path.display(),
            mode
        );
    }

    let raw = fs::read_to_string(&path)?;
    let parsed: AuthFile = toml::from_str(&raw)?;

    if parsed.auth.uid.is_empty() {
        return Err(Error::MissingAuthKey("uid"));
    }
    if parsed.auth.pass_cookie.is_empty() {
        return Err(Error::MissingAuthKey("pass"));
    }

    Ok(parsed.auth)
}

pub fn write_auth_file(auth: &AuthConfig) -> Result<PathBuf> {
    ensure_state_dir_permissions()?;
    let path = auth_file_path()?;

    let doc = AuthWrite {
        auth: AuthWriteSection {
            uid: &auth.uid,
            pass_cookie: &auth.pass_cookie,
            cf_clearance: &auth.cf_clearance,
        },
    };
    let contents = toml::to_string(&doc)?;
    fs::write(&path, contents)?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    Ok(path)
}

pub fn parse_cookie_string_arg(arg: &str) -> Result<AuthConfig> {
    let raw = if arg == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf.trim().to_string()
    } else {
        arg.to_string()
    };

    let mut cookies: HashMap<String, String> = HashMap::new();
    for parsed in Cookie::split_parse(raw) {
        let Ok(cookie) = parsed else {
            continue;
        };
        cookies.insert(
            cookie.name().trim().to_string(),
            cookie.value().trim().to_string(),
        );
    }

    let uid = cookies.get("uid").cloned().unwrap_or_default();
    let pass_cookie = cookies.get("pass").cloned().unwrap_or_default();
    let cf_clearance = cookies
        .get("cf_clearance")
        .map(|v| v.to_string())
        .filter(|v| !v.is_empty());

    let mut missing = Vec::new();
    if uid.is_empty() {
        missing.push("uid");
    }
    if pass_cookie.is_empty() {
        missing.push("pass");
    }

    if !missing.is_empty() {
        return Err(Error::MissingCookieKeys(missing.join(", ")));
    }

    Ok(AuthConfig {
        uid,
        pass_cookie,
        cf_clearance,
    })
}

#[cfg(test)]
mod tests {
    use super::parse_cookie_string_arg;

    #[test]
    fn parse_cookie_string_works() {
        let auth = parse_cookie_string_arg("uid=123; pass=abc; cf_clearance=xyz").unwrap();
        assert_eq!(auth.uid, "123");
        assert_eq!(auth.pass_cookie, "abc");
        assert_eq!(auth.cf_clearance.as_deref(), Some("xyz"));
    }

    #[test]
    fn parse_cookie_missing_required_key_errors() {
        let err = parse_cookie_string_arg("uid=123").unwrap_err();
        assert!(err.to_string().contains("missing required key"));
    }

    #[test]
    fn parse_cookie_string_tolerates_noise() {
        let auth = parse_cookie_string_arg("uid=123; junk; pass=abc; cf_clearance=xyz").unwrap();
        assert_eq!(auth.uid, "123");
        assert_eq!(auth.pass_cookie, "abc");
        assert_eq!(auth.cf_clearance.as_deref(), Some("xyz"));
    }
}
