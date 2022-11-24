//! Error types

use alloc::string::String;
use core::num::TryFromIntError;

use displaydoc::Display;

use crate::{account, vote};

#[derive(Debug, Display, Clone)]
pub enum Error {
    /// cryptographic error
    Crypto,
    /// invalid key: `{detail}`
    InvalidKey { detail: String },
    /// length error
    Length,
    /// error parsing data: `{data}`
    Parse { data: String },
    /// error parsing int data: `{data}`
    ParseInt {
        data: String,
        e: core::num::ParseIntError,
    },
    /// protocol error: `{detail}`
    Protocol { detail: String },
    /// date out of range
    DateOutOfRange,
    /// duration value out of range
    DurationOutOfRange,
    /// empty signature
    EmptySignature,
    /// bad signature: `{detail}`
    SignatureInvalid { detail: String },
    /// invalid message type
    InvalidMessageType,
    /// negative height
    NegativeHeight(TryFromIntError),
    /// negative round
    NegativeRound(TryFromIntError),
    /// negative POL round
    NegativePolRound,
    /// negative validator index
    NegativeValidatorIndex(TryFromIntError),
    /// invalid hash: expected hash size to be 32 bytes
    InvalidHashSize,
    /// absent commitsig has non-zero timestamp
    NonZeroTimestamp,
    /// invalid account ID length
    InvalidAccountIdLength,
    /// invalid signature ID length
    InvalidSignatureIdLength,
    /// integer overflow
    IntegerOverflow(TryFromIntError),
    /// timestamp nanosecond component is out of range
    TimestampNanosOutOfRange,
    /// timestamp conversion error
    TimestampConversion,
    /// no vote found
    NoVoteFound,
    /// no proposal found
    NoProposalFound,
    /// invalid app hash length
    InvalidAppHashLength,
    /// invalid part set header
    InvalidPartSetHeader { detail: String },
    /// missing header field
    MissingHeader,
    /// missing data field
    MissingData,
    /// missing evidence field
    MissingEvidence,
    /// missing timestamp field
    MissingTimestamp,
    /// missing version
    MissingVersion,
    /// missing max_age_duration
    MissingMaxAgeDuration,
    /// missing public key
    MissingPublicKey,
    /// missing validator
    MissingValidator,
    /// missing last commit info
    MissingLastCommitInfo,
    /// missing genesis time
    MissingGenesisTime,
    /// missing consensus params
    MissingConsensusParams,
    /// invalid timestamp: `{reason}`
    InvalidTimestamp { reason: String },
    /// invalid block: `{reason}`
    InvalidBlock { reason: String },
    /// last_block_id is not null on first height
    InvalidFirstHeader,
    /// invalid signature: `{reason}`
    InvalidSignature { reason: String },
    /// invalid validator address
    InvalidValidatorAddress,
    /// invalid signed header
    InvalidSignedHeader,
    /// invalid evidence
    InvalidEvidence,
    /// invalid validator parameters
    InvalidValidatorParams,
    /// invalid version parameters
    InvalidVersionParams,
    /// invalid ABCI request type
    InvalidAbciRequestType,
    /// invalid ABCI response type
    InvalidAbciResponseType,
    /// invalid block id flag
    BlockIdFlag,
    /// negative power
    NegativePower(TryFromIntError),
    /// unsupported key type
    UnsupportedKeyType,
    /// unsupported CheckTx type
    UnsupportedCheckTxType,
    /// unsupported ApplySnapshotChunkResult type
    UnsupportedApplySnapshotChunkResult,
    /// unsupported OfferSnapshotChunkResult type
    UnsupportedOfferSnapshotChunkResult,
    /// mismatch between raw voting (`{raw:?}`) and computed one (`{computed:?}`)
    RawVotingPowerMismatch {
        raw: vote::Power,
        computed: vote::Power,
    },
    /// negative max_age_num_blocks
    NegativeMaxAgeNum(TryFromIntError),
    /// proposer with address `{account}` no found in validator set
    ProposerNotFound { account: account::Id },
    /// time parsing error
    TimeParse(time::error::Parse),
    /// subtle encoding error
    SubtleEncoding(subtle_encoding::Error),
    /// signature error
    Signature,
    /// trust threshold is too large (must be <= 1)
    TrustThresholdTooLarge,
    /// undefined trust threshold (denominator cannot be 0)
    UndefinedTrustThreshold,
    /// trust threshold too small (must be >= 1/3)
    TrustThresholdTooSmall,
}

impl From<core::convert::Infallible> for Error {
    fn from(_never: core::convert::Infallible) -> Error {
        unreachable!("Infallible can never be constructed")
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn flex_error::StdError + 'static)> {
        match &self {
            Error::ParseInt { e, .. } => Some(e),
            Error::NegativeHeight(e) => Some(e),
            Error::NegativeRound(e) => Some(e),
            Error::NegativeValidatorIndex(e) => Some(e),
            Error::IntegerOverflow(e) => Some(e),
            Error::NegativePower(e) => Some(e),
            Error::NegativeMaxAgeNum(e) => Some(e),
            Error::TimeParse(e) => Some(e),
            Error::SubtleEncoding(e) => Some(e),
            _ => None,
        }
    }
}
