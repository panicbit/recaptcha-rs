use reqwest::Url;
use lazy_static::lazy_static;
use crate::Client;
use crate::client::Secret;

lazy_static! {
    static ref DEFAULT_SITEVERIFY_URL: Url =
        Url::parse("https://www.google.com/recaptcha/api/siteverify").unwrap();
}

pub struct Builder {
    http: Option<reqwest::Client>,
    siteverify_url: Url,

}

impl Builder {
    pub fn new() -> Self {
        Self {
            http: None,
            siteverify_url: DEFAULT_SITEVERIFY_URL.clone(),
        }
    }

    pub fn set_http_client(mut self, http: reqwest::Client) -> Self {
        self.http = Some(http);
        self
    }

    pub fn set_host(mut self, host: &str) -> Result<Self, SetHostError> {
        self.siteverify_url.set_host(Some(host))?;
        Ok(self)
    }

    pub fn build(self, secret: String) -> Client {
        let secret = Secret::new(secret);

        Client {
            http: self.http.unwrap_or_else(reqwest::Client::new),
            siteverify_url: self.siteverify_url,
            secret,
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to set host: {0}")]
pub struct SetHostError(#[from] url::ParseError);
