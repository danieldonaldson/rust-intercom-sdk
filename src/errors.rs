use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotLoggedIn,
    HTTPError(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotLoggedIn => write!(f, "Not logged in"),
            Error::HTTPError(resp) => write!(f, "HTTP request to Intercom failed {}", resp),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HTTPError(e.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        Error::HTTPError(e.to_string())
    }
}
