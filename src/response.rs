use log::trace;
use serde::de::DeserializeOwned;

use crate::RequestError;

pub struct Response<T> {
    pub result: Result<T, RequestError>,
    pub response_json: String,
    pub rate_limit_remaining: usize,
    pub rate_limit_requests_limit: usize,
}

impl<T: DeserializeOwned> Response<T> {
    pub fn new(raw_json: String, allowed: usize, remaining: usize) -> Response<T> {
        Self {
            result: try_parse::<T>(&raw_json),
            response_json: raw_json,
            rate_limit_remaining: remaining,
            rate_limit_requests_limit: allowed,
        }
    }

    pub fn try_parse(&self) -> Result<T, RequestError> {
        try_parse::<T>(&self.response_json)
    }
}

pub fn try_parse<T: DeserializeOwned>(word_json: &str) -> Result<T, RequestError> {
    let result: Result<T, serde_json::Error> = serde_json::from_str::<T>(word_json);
    match result {
        Ok(word_data) => Ok(word_data),
        Err(e) => {
            trace!("serde says {}", e);
            Err(RequestError::ResultParseError)
        }
    }
}
