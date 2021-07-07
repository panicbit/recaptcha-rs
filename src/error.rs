use std::collections::HashSet;
use std::io;
use serde::{Deserializer, Deserialize};
use reqwest;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Codes(HashSet<Code>),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    Io(#[from] io::Error),
}

#[derive(PartialEq, Eq, Hash, Debug)]
/// Error code that indicates the reason for a failed verification.
///
/// See also the [recaptcha error code reference](https://developers.google.com/recaptcha/docs/verify#error_code_reference).
pub enum Code {
    /// The secret parameter is missing.
    MissingSecret,
    /// The secret parameter is invalid or malformed.
    InvalidSecret,
    /// The response parameter is missing.
    MissingResponse,
    /// The response parameter is invalid or malformed.
    InvalidResponse,
    /// The request is invalid or malformed.
    BadRequest,
    // The response is no longer valid: either is too old or has been used previously.
    TimeoutOrDuplicate,
    Unknown(String)
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let code = String::deserialize(de)?;
        Ok(match &*code {
            "missing-input-secret" => Code::MissingSecret,
            "invalid-input-secret" => Code::InvalidSecret,
            "missing-input-response" => Code::MissingResponse,
            "invalid-input-response" => Code::InvalidResponse,
            "bad-request" => Code::BadRequest,
            "timeout-or-duplicate" => Code::TimeoutOrDuplicate,
            _ => Code::Unknown(code),
        })
    }
}
