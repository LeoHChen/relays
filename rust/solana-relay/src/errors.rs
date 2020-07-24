use bitcoin_spv::types::SPVError;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use solana_sdk::{
    decode_error::DecodeError,
    info,
    program_error::{PrintProgramError, ProgramError},
};

use thiserror::Error;

/// Errors that may be returned by the TokenSwap program.
///
/// Most of these are copied directly from bitcoin spv
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum RelayError {
    // TODO: more here
    /// Relay is already init
    #[error("AlreadyInit")]
    AlreadyInit,

    /// State account has insufficient space to update the state
    #[error("InsufficientStateSpace")]
    InsufficientStateSpace,

    // Below copied from bitcoin-spv
    /// Overran a checked read on a slice
    #[error("ReadOverrun")]
    ReadOverrun,
    /// Attempted to parse a CompactInt without enough bytes
    #[error("BadCompactInt")]
    BadCompactInt,
    /// Called `extract_op_return_data` on an output without an op_return.
    #[error("MalformattedOpReturnOutput")]
    MalformattedOpReturnOutput,
    /// `extract_hash` identified a SH output prefix without a SH postfix.
    #[error("MalformattedP2SHOutput")]
    MalformattedP2SHOutput,
    /// `extract_hash` identified a PKH output prefix without a PKH postfix.
    #[error("MalformattedP2PKHOutput")]
    MalformattedP2PKHOutput,
    /// `extract_hash` identified a Witness output with a bad length tag.
    #[error("MalformattedWitnessOutput")]
    MalformattedWitnessOutput,
    /// `extract_hash` could not identify the output type.
    #[error("MalformattedOutput")]
    MalformattedOutput,
    /// Header not exactly 80 bytes.
    #[error("WrongLengthHeader")]
    WrongLengthHeader,
    /// Header chain changed difficulties unexpectedly
    #[error("UnexpectedDifficultyChange")]
    UnexpectedDifficultyChange,
    /// Header does not meet its own difficulty target.
    #[error("InsufficientWork")]
    InsufficientWork,
    /// Header in chain does not correctly reference parent header.
    #[error("InvalidChain")]
    InvalidChain,
    /// When validating a `BitcoinHeader`, the `hash` field is not the digest
    /// of the raw header.
    #[error("WrongDigest")]
    WrongDigest,
    /// When validating a `BitcoinHeader`, the `merkle_root` field does not
    /// match the root found in the raw header.
    #[error("WrongMerkleRoot")]
    WrongMerkleRoot,
    /// When validating a `BitcoinHeader`, the `prevhash` field does not
    /// match the parent hash found in the raw header.
    #[error("WrongPrevHash")]
    WrongPrevHash,
    /// A `vin` (transaction input vector) is malformatted.
    #[error("InvalidVin")]
    InvalidVin,
    /// A `vout` (transaction output vector) is malformatted.
    #[error("InvalidVout")]
    InvalidVout,
    /// When validating an `SPVProof`, the `tx_id` field is not the digest
    /// of the `version`, `vin`, `vout`, and `locktime`.
    #[error("WrongTxID")]
    WrongTxID,
    /// When validating an `SPVProof`, the `intermediate_nodes` is not a valid
    /// merkle proof connecting the `tx_id_le` to the `confirming_header`.
    #[error("BadMerkleProof")]
    BadMerkleProof,
    /// TxOut's reported length does not match passed-in byte slice's length
    #[error("OutputLengthMismatch")]
    OutputLengthMismatch,
    /// Any other error
    #[error("UnknownError")]
    UnknownError,
}

impl From<SPVError> for RelayError {
    fn from(e: SPVError) -> RelayError {
        match e {
            SPVError::ReadOverrun => RelayError::ReadOverrun,
            SPVError::BadCompactInt => RelayError::BadCompactInt,
            SPVError::MalformattedOpReturnOutput => RelayError::MalformattedOpReturnOutput,
            SPVError::MalformattedP2SHOutput => RelayError::MalformattedP2SHOutput,
            SPVError::MalformattedP2PKHOutput => RelayError::MalformattedP2PKHOutput,
            SPVError::MalformattedWitnessOutput => RelayError::MalformattedWitnessOutput,
            SPVError::MalformattedOutput => RelayError::MalformattedOutput,
            SPVError::WrongLengthHeader => RelayError::WrongLengthHeader,
            SPVError::UnexpectedDifficultyChange => RelayError::UnexpectedDifficultyChange,
            SPVError::InsufficientWork => RelayError::InsufficientWork,
            SPVError::InvalidChain => RelayError::InvalidChain,
            SPVError::WrongDigest => RelayError::WrongDigest,
            SPVError::WrongMerkleRoot => RelayError::WrongMerkleRoot,
            SPVError::WrongPrevHash => RelayError::WrongPrevHash,
            SPVError::InvalidVin => RelayError::InvalidVin,
            SPVError::InvalidVout => RelayError::InvalidVout,
            SPVError::WrongTxID => RelayError::WrongTxID,
            SPVError::BadMerkleProof => RelayError::BadMerkleProof,
            SPVError::OutputLengthMismatch => RelayError::OutputLengthMismatch,
            SPVError::UnknownError => RelayError::UnknownError,
        }
    }
}

impl From<RelayError> for ProgramError {
    fn from(e: RelayError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for RelayError {
    fn type_of() -> &'static str {
        "Relay Error"
    }
}

impl PrintProgramError for RelayError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            RelayError::AlreadyInit => info!("RelayError: AlreadyInit"),
            RelayError::InsufficientStateSpace => info!("RelayError: InsufficientStateSpace"),
            RelayError::ReadOverrun => info!("RelayError: ReadOverrun"),
            RelayError::BadCompactInt => info!("RelayError: BadCompactInt"),
            RelayError::MalformattedOpReturnOutput => {
                info!("RelayError: MalformattedOpReturnOutput")
            }
            RelayError::MalformattedP2SHOutput => info!("RelayError: MalformattedP2SHOutput"),
            RelayError::MalformattedP2PKHOutput => info!("RelayError: MalformattedP2PKHOutput"),
            RelayError::MalformattedWitnessOutput => info!("RelayError: MalformattedWitnessOutput"),
            RelayError::MalformattedOutput => info!("RelayError: MalformattedOutput"),
            RelayError::WrongLengthHeader => info!("RelayError: WrongLengthHeader"),
            RelayError::UnexpectedDifficultyChange => {
                info!("RelayError: UnexpectedDifficultyChange")
            }
            RelayError::InsufficientWork => info!("RelayError: InsufficientWork"),
            RelayError::InvalidChain => info!("RelayError: InvalidChain"),
            RelayError::WrongDigest => info!("RelayError: WrongDigest"),
            RelayError::WrongMerkleRoot => info!("RelayError: WrongMerkleRoot"),
            RelayError::WrongPrevHash => info!("RelayError: WrongPrevHash"),
            RelayError::InvalidVin => info!("RelayError: InvalidVin"),
            RelayError::InvalidVout => info!("RelayError: InvalidVout"),
            RelayError::WrongTxID => info!("RelayError: WrongTxID"),
            RelayError::BadMerkleProof => info!("RelayError: BadMerkleProof"),
            RelayError::OutputLengthMismatch => info!("RelayError: OutputLengthMismatch"),
            RelayError::UnknownError => info!("RelayError: UnknownError"),
        }
    }
}