use bytes::Bytes;

use crate::prelude::*;

#[doc = include_str!("../doc/request-checktx.md")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CheckTx {
    /// The transaction bytes.
    pub tx: Bytes,
    /// The kind of check to perform.
    ///
    /// Note: this field is called `type` in the protobuf, but we call it `kind`
    /// to avoid the Rust keyword.
    pub kind: CheckTxKind,
}

/// The possible kinds of [`CheckTx`] checks.
///
/// Note: the
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
/// calls this `CheckTxType`, but we follow the Rust convention and name it `CheckTxKind`
/// to avoid confusion with Rust types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CheckTxKind {
    /// A full check is required (the default).
    New = 0,
    /// Indicates that the mempool is initiating a recheck of the transaction.
    Recheck = 1,
}

impl Default for CheckTxKind {
    fn default() -> Self {
        CheckTxKind::New
    }
}

// =============================================================================
// Protobuf conversions
// =============================================================================

use core::convert::TryFrom;

use tendermint_proto::{abci as pb, Protobuf};

impl From<CheckTx> for pb::RequestCheckTx {
    fn from(check_tx: CheckTx) -> Self {
        Self {
            tx: check_tx.tx,
            r#type: check_tx.kind as i32,
        }
    }
}

impl TryFrom<pb::RequestCheckTx> for CheckTx {
    type Error = crate::Error;

    fn try_from(check_tx: pb::RequestCheckTx) -> Result<Self, Self::Error> {
        let kind = match check_tx.r#type {
            0 => CheckTxKind::New,
            1 => CheckTxKind::Recheck,
            _ => return Err(crate::Error::UnsupportedCheckTxType),
        };
        Ok(Self {
            tx: check_tx.tx,
            kind,
        })
    }
}

impl Protobuf<pb::RequestCheckTx> for CheckTx {}
