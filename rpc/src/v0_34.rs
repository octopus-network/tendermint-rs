//! Support for Tendermint RPC version 0.34.
//!
//! The API in this module provides compatibility with the Tendermint RPC
//! protocol as implemented in [Tendermint Core][tendermint] version 0.34.
//!
//! [tendermint]: https://github.com/tendermint/tendermint

mod event;

pub use event::Event;
