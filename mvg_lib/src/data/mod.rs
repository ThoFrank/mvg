pub mod location;
pub mod departure;

#[derive(Debug)]
pub enum MVGError {
    HyperError(hyper::Error),
    JsonError(serde_json::Error),
    InvalidUri(http::uri::InvalidUri),
    ArgumentError(String),
}

impl From<hyper::Error> for MVGError {
    fn from(error: hyper::Error) -> Self {
        Self::HyperError(error)
    }
}

impl From<serde_json::Error> for MVGError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(error)
    }
}

impl From<http::uri::InvalidUri> for MVGError {
    fn from(error: http::uri::InvalidUri) -> Self {
        Self::InvalidUri(error)
    }
}
