use core::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum RequestError {
    RequestError,
    ResultParseError,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestError::RequestError => f.write_str("RequestError"),
            RequestError::ResultParseError => f.write_str("ResultParseError"),
        }
    }
}

impl StdError for RequestError {
    fn description(&self) -> &str {
        match *self {
            RequestError::RequestError => "WordAPI request failed",
            RequestError::ResultParseError => "Could not parse result",
        }
    }
}
