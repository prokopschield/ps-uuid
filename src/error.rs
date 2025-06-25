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
