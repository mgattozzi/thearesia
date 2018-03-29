use std::io::Error;
use github_rs as gh;
use hyper;
use serde_json;

#[derive(Debug, Fail, Serialize, Deserialize)]
pub enum ThearesiaFailure {
    #[fail(display = "github-rs failed error was: {}", error)]
    Github {
        error: String,
    },
    #[fail(display = "IO error occured it was: {}", error)]
    Io {
        error: String,
    },
    #[fail(display = "Hyper error occured it was: {}", error)]
    Hyper {
        error: String,
    },
    #[fail(display = "Serde error occured it was: {}", error)]
    Serde {
        error: String,
    },
    #[fail(display = "Status Code error occured it was: {}", error)]
    StatusCodeFail {
        error: String
    }
}

impl From<Error> for ThearesiaFailure {
    fn from(e: Error) -> Self {
        ThearesiaFailure::Io{ error: e.to_string() }
    }
}

impl From<gh::errors::Error> for ThearesiaFailure {
    fn from(e: gh::errors::Error) -> Self {
        ThearesiaFailure::Github{ error: e.to_string() }
    }
}

impl From<serde_json::Error> for ThearesiaFailure {
    fn from(e: serde_json::Error) -> Self {
        ThearesiaFailure::Hyper{ error: e.to_string() }
    }
}
impl From<hyper::error::UriError> for ThearesiaFailure {
    fn from(e: hyper::error::UriError) -> Self {
        ThearesiaFailure::Hyper{ error: e.to_string() }
    }
}

impl From<hyper::Error> for ThearesiaFailure {
    fn from(e: hyper::Error) -> Self {
        ThearesiaFailure::Hyper{ error: e.to_string() }
    }
}
