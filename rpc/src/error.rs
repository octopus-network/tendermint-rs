//! JSON-RPC error types

use core::time::Duration;

use displaydoc::Display;

use crate::{prelude::*, response_error::ResponseError, rpc_url::Url};

#[derive(Debug, Display, Clone)]
/// No source
pub struct NoSource;

#[cfg(feature = "http")]
type HttpError = http::Error;

#[cfg(not(feature = "http"))]
type HttpError = NoSource;

#[cfg(feature = "http")]
type InvalidUriError = http::uri::InvalidUri;

#[cfg(not(feature = "http"))]
type InvalidUriError = NoSource;

#[cfg(feature = "hyper")]
type HyperError = hyper::Error;

#[cfg(not(feature = "hyper"))]
type HyperError = NoSource;

#[cfg(feature = "tokio")]
type JoinError = tokio::task::JoinError;

#[cfg(not(feature = "tokio"))]
type JoinError = NoSource;

#[cfg(feature = "async-tungstenite")]
type TungsteniteError = async_tungstenite::tungstenite::Error;

#[cfg(not(feature = "async-tungstenite"))]
type TungsteniteError = NoSource;

#[derive(Debug, Display)]
pub enum Error {
    /// response error
    Response(ResponseError),
    /// I/O error
    Io(std::io::Error),
    /// HTTP error
    Http(HttpError),
    /// Hyper error
    Hyper(HyperError),
    /// invalid params error: `{message}`
    InvalidParams { message: String },
    /// web socket error: `{message}`
    WebSocket {
        message: String,
        error: TungsteniteError,
    },
    /// reading from WebSocket connection timed out after `{timeout:?}` seconds
    WebSocketTimeout { timeout: Duration },
    /// method not found: `{method}`
    MethodNotFound { method: String },
    /// parse error: `{reason}`
    Parse { reason: String },
    /// client internal error: `{reason}`
    ClientInternal { reason: String },
    /// timed out waiting for healthy response after `{duration:?}`ms
    Timeout { duration: Duration },
    /// failed to send message to internal channel
    ChannelSend,
    /// cannot use URL `{url}` with HTTP clients
    InvalidUrl { url: Url },
    /// invalid URI
    InvalidUri(InvalidUriError),
    /// tendermint error
    Tendermint(tendermint::Error),
    /// error parsing integer
    ParseInt(core::num::ParseIntError),
    /// number out of range
    OutOfRange(core::num::TryFromIntError),
    /// only TCP-based node addresses are supported
    InvalidNetworkAddress,
    /// no matching response for incoming request
    MismatchResponse,
    /// unrecognized event type: `{event_type}`
    UnrecognizedEventType { event_type: String },
    /// serde parse error
    Serde(serde_json::Error),
    /// parse error
    ParseUrl(url::ParseError),
    /// tungstenite error
    Tungstenite(TungsteniteError),
    /// join error
    Join(JoinError),
    /// server returned malformatted JSON (no 'result' or 'error')
    MalformedJson,
    /// unsupported scheme: `{scheme}`
    UnsupportedScheme { scheme: String },
    /// server RPC version unsupported: `{version}` (only  `{supported}` supported)
    UnsupportedRpcVersion { version: String, supported: String },
}

#[cfg(feature = "tokio")]
impl Error {
    pub fn send<T>(_: tokio::sync::mpsc::error::SendError<T>) -> Error {
        Error::ChannelSend
    }
}
