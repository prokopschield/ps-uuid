#![allow(clippy::module_name_repetitions)]
use std::num::TryFromIntError;

use thiserror::Error;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UuidConstructionError {
    #[error(transparent)]
    IntegerConversion(#[from] TryFromIntError),

    #[error("The timestamp provided is too low.")]
    TimestampBeforeEpoch,

    #[error("The timestamp provided is too high.")]
    TimestampOverflow,
}

#[derive(thiserror::Error, Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DurationToTicksError {
    #[default]
    #[error("The timestamp provided is too high.")]
    TimestampOverflow,
}

impl From<DurationToTicksError> for UuidConstructionError {
    fn from(_: DurationToTicksError) -> Self {
        Self::TimestampOverflow
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UuidParseError {
    #[error("invalid length")]
    InvalidLength,

    #[error("invalid character `{ch}` at index {idx}")]
    InvalidCharacter { ch: char, idx: usize },

    #[error("hyphens are in the wrong position")]
    InvalidHyphenPlacement,

    #[error("mismatching or misplaced braces")]
    InvalidBraces,
}
