use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct SearchError {
    details: String,
}

impl SearchError {
    pub fn new(msg: &str) -> SearchError {
        SearchError { details: msg.to_string() }
    }
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SearchError {}