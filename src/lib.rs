extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_secret_missing_response() {
        use error::Error::*;
        use error::Code::*;
        let response = verify("", "", None).await;

        match response {
            Ok(()) => panic!("unexpected response: Ok(())"),
            Err(Codes(ref errors)) => {
                assert!(errors.contains(&InvalidSecret));
            }
            Err(e) => panic!("unexpected error: {}", e),
        };

        println!("{:?}", response);
    }
}
