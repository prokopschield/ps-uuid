#![allow(clippy::module_name_repetitions)]
use std::num::TryFromIntError;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UuidConstructionError {
    #[error(transparent)]
    IntegerConversion(#[from] TryFromIntError),

    #[error("The timestamp provided is too low.")]
    TimestampBeforeEpoch,

    #[error("The timestamp provided is too high.")]
    TimestampOverflow,
}
