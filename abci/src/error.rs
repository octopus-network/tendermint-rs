//! tendermint-abci errors

use displaydoc::Display;
use tendermint_proto::abci::response::Value;

#[derive(Debug, Display)]
pub enum Error {
    /// I/O error
    Io(std::io::Error),
    /// error encoding protocol buffer
    Encode(prost::EncodeError),
    /// error encoding protocol buffer
    Decode(prost::DecodeError),
    /// server connection terminated
    ServerConnectionTerminated,
    /// malformed server response
    MalformedServerResponse,
    /// unexpected server response type: expected `{expected}`, but got `{got:?}`
    UnexpectedServerResponseType { expected: String, got: Value },
    /// channel send error
    ChannelSend,
    /// channel recv error
    ChannelRecv(std::sync::mpsc::RecvError),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn flex_error::StdError + 'static)> {
        match &self {
            Error::Io(e) => Some(e),
            Error::Encode(e) => Some(e),
            Error::Decode(e) => Some(e),
            Error::ChannelRecv(e) => Some(e),
            _ => None,
        }
    }
}

impl Error {
    pub fn send<T>(_e: std::sync::mpsc::SendError<T>) -> Error {
        Error::ChannelSend
    }
}
