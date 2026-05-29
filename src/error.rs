#![allow(clippy::module_name_repetitions)]
use std::num::TryFromIntError;

use thiserror::Error;

/// An error that occurs while constructing a [`UUID`](crate::UUID).
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UuidConstructionError {
    /// An integer conversion failed.
    #[error(transparent)]
    IntegerConversion(#[from] TryFromIntError),

    /// The timestamp provided precedes the relevant epoch.
    #[error("The timestamp provided is too low.")]
    TimestampBeforeEpoch,

    /// The timestamp provided exceeds the representable range.
    #[error("The timestamp provided is too high.")]
    TimestampOverflow,
}

/// An error that occurs while converting a [`Duration`](std::time::Duration) to UUID timestamp ticks.
#[derive(thiserror::Error, Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DurationToTicksError {
    /// The duration exceeds the representable range.
    #[default]
    #[error("The timestamp provided is too high.")]
    TimestampOverflow,
}

impl From<DurationToTicksError> for UuidConstructionError {
    fn from(_: DurationToTicksError) -> Self {
        Self::TimestampOverflow
    }
}

/// An error that occurs while parsing a [`UUID`](crate::UUID) from a string.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UuidParseError {
    /// The input has an invalid length.
    #[error("invalid length")]
    InvalidLength,

    /// The input contains an invalid character.
    #[error("invalid character `{ch}` at index {idx}")]
    InvalidCharacter {
        /// The offending character.
        ch: char,
        /// The index of the offending character.
        idx: usize,
    },

    /// One or more hyphens are in the wrong position.
    #[error("hyphens are in the wrong position")]
    InvalidHyphenPlacement,

    /// The braces are mismatched or misplaced.
    #[error("mismatching or misplaced braces")]
    InvalidBraces,
}
