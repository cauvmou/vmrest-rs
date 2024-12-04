use crate::VMRestAPIError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Reqwest(reqwest::Error),
    VMRest(VMRestAPIError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Json(inner) => inner.fmt(f),
            Error::Reqwest(inner) => inner.fmt(f),
            Error::VMRest(inner) => inner.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<VMRestAPIError> for Error {
    fn from(value: VMRestAPIError) -> Self {
        Self::VMRest(value)
    }
}
