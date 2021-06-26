use std::collections::HashSet;
use std::net::IpAddr;

use reqwest::Url;

use crate::Error;
use crate::response::RecaptchaResponse;

mod secret;
use secret::Secret;

mod builder;
pub use builder::Builder;

#[derive(Debug)]
pub struct Client {
    http: reqwest::Client,
    siteverify_url: Url,
    secret: Secret,
}

impl Client {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub async fn verify(&self, response: &str, user_ip: Option<&IpAddr>) -> Result<(), Error> {
        let url = self.siteverify_url.clone();

        let parameters = SiteverifyParameters {
            secret: self.secret.reveal(),
            response,
            remoteip: user_ip,
        };

        let response = self.http.post(url)
            .form(&parameters)
            .send()
            .await?;

        let recaptcha_response = response.json::<RecaptchaResponse>().await?;

        match (recaptcha_response.success, recaptcha_response.error_codes) {
            (true, _) => Ok(()),
            (false, Some(errors)) => Err(Error::Codes(errors)),
            (false, _) => Err(Error::Codes(HashSet::new()))
        }
    }
}

#[derive(Serialize)]
struct SiteverifyParameters<'a> {
    secret: &'a str,
    response: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    remoteip: Option<&'a IpAddr>,
}
