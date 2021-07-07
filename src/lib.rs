extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::net::IpAddr;

pub use error::Error;

pub mod error;
mod response;

mod client;
pub use client::{Client, Builder};

/// Verify a recaptcha user response
pub async fn verify(secret: &str, response: &str, user_ip: Option<&IpAddr>) -> Result<(), Error> {
    let client = Client::builder().build(secret.into());

    client.verify(response, user_ip).await
}
