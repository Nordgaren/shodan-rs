use crate::error::ShodanError::*;
use crate::response::{ErrorResponse, ShodanClientResponse};

#[derive(Debug)]
pub enum ShodanError {
    ShodanClientError(String),
    IOError(std::io::Error),
    ReqwestError(reqwest::Error),
}

impl From<std::io::Error> for ShodanError {
    fn from(e: std::io::Error) -> Self {
        IOError(e)
    }
}

impl From<reqwest::Error> for ShodanError {
    fn from(e: reqwest::Error) -> Self {
        ReqwestError(e)
    }
}
