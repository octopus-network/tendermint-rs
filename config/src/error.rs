use std::io::Error as IoError;

use displaydoc::Display;
use tendermint::Error as TendermintError;

#[derive(Debug, Display)]
pub enum Error {
    /// I/O error
    Io(IoError),
    /// failed to open file: `{path}`
    FileIo { path: String, e: IoError },
    /// error parsing data: `{data}`
    Parse { data: String },
    /// serde json error
    SerdeJson(serde_json::Error),
    /// toml de error
    Toml(toml::de::Error),
    /// error parsing url error
    ParseUrl(url::ParseError),
    /// endermint error
    Tendermint(TendermintError),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Error::Io(e) => Some(e),
            Error::FileIo { e, .. } => Some(e),
            Error::Parse { .. } => None,
            Error::SerdeJson(e) => Some(e),
            Error::Toml(e) => Some(e),
            Error::ParseUrl(e) => Some(e),
            Error::Tendermint(e) => Some(e),
        }
    }
}
