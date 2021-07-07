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
pub enum Code {
    MissingSecret,
    InvalidSecret,
    MissingResponse,
    InvalidResponse,
    BadRequest,
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
