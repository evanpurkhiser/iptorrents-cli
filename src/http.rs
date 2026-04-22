use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, COOKIE, HeaderMap, HeaderValue, USER_AGENT};

use crate::error::{Error, Result};
use crate::models::AuthConfig;

pub const BASE_URL: &str = "https://iptorrents.com";

const FIREFOX_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0";

#[derive(Debug, Clone)]
pub struct IptClient {
    client: Client,
    cookie_header: HeaderValue,
    base_url: String,
}

impl IptClient {
    pub fn new(auth: AuthConfig, base_url: String) -> Result<Self> {
        if auth.uid.is_empty() || auth.pass_cookie.is_empty() {
            return Err(Error::EmptyAuthValues);
        }

        let mut default_headers = HeaderMap::new();
        default_headers.insert(USER_AGENT, HeaderValue::from_static(FIREFOX_USER_AGENT));
        default_headers.insert(
            ACCEPT,
            HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            ),
        );
        default_headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));

        let client = Client::builder().default_headers(default_headers).build()?;

        let mut cookie = format!("uid={}; pass={}", auth.uid, auth.pass_cookie);
        if let Some(cf) = auth.cf_clearance {
            cookie.push_str("; cf_clearance=");
            cookie.push_str(&cf);
        }

        let cookie_header = HeaderValue::from_str(&cookie)
            .map_err(|e| Error::InvalidCookieHeader(e.to_string()))?;

        Ok(Self {
            client,
            cookie_header,
            base_url,
        })
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.client
            .get(format!("{}{}", self.base_url, path))
            .header(COOKIE, self.cookie_header.clone())
    }

    pub fn get_absolute(&self, url: &str) -> RequestBuilder {
        self.client
            .get(url)
            .header(COOKIE, self.cookie_header.clone())
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

pub fn verify_session(client: &IptClient) -> Result<()> {
    let response = client.get("/t").send()?;
    let status_ok = response.status().as_u16() == 200;
    let body = response.text()?.to_ascii_lowercase();
    if !status_ok || body.contains("sign in") {
        return Err(Error::InvalidSession);
    }
    Ok(())
}
