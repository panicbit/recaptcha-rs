use reqwest;
use serde::{Deserialize, Deserializer};
use std::collections::HashSet;
use std::{fmt, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Codes(HashSet<Code>),
    #[error("{0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0:?}")]
    Io(#[from] io::Error),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Code {
    MissingSecret,
    InvalidSecret,
    MissingResponse,
    InvalidResponse,
    BadRequest,
    TimeoutOrDuplicate,
    Unknown(String),
}

impl fmt::Display for Code {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Code::BadRequest => write!(f, "bad-request"),
            Code::InvalidResponse => write!(f, "invalid-input-secret"),
            Code::InvalidSecret => write!(f, "invalid-input-secret"),
            Code::MissingResponse => write!(f, "missing-input-response"),
            Code::MissingSecret => write!(f, "missing-input-secret"),
            Code::TimeoutOrDuplicate => write!(f, "timeout-or-duplicate"),
            Code::Unknown(s) => write!(f, "unknown code: {}", s),
        }
    }
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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
