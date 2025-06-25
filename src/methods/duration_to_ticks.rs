use std::time::Duration;

use crate::{DurationToTicksError, UUID};

impl UUID {
    /// Generates an RFC 4122 timestamp from a `Duration`.
    ///
    /// The timestamp represents the number of 100-nanosecond intervals since
    /// the Gregorian epoch (1582-10-15 00:00:00 UTC).
    ///
    /// # Returns
    ///
    /// The number of full 100-ns segments is returned as a `u64`.
    ///
    /// # Errors
    ///
    /// - [`DurationToTicksError::TimestampOverflow`] is returned if the
    ///   duration corresponds to a value of \( 2^{60} \) or greater, which
    ///   would overflow the 60 bits available for the timestamp. This occurs
    ///   for dates beyond 5236-03-31.
    pub const fn duration_to_ticks(duration: Duration) -> Result<u64, DurationToTicksError> {
        // `duration.as_nanos()` returns a `u128`, preventing overflow for
        // large durations during this intermediate calculation.
        let ticks = duration.as_nanos() / 100;

        // The UUID timestamp is 60 bits. We must check if the calculated
        // ticks can fit. `1u128 << 60` is the first value that is too large.
        if ticks >= (1u128 << 60) {
            return Err(DurationToTicksError::TimestampOverflow);
        }

        // The value is confirmed to be < 2^60, so it can be safely cast to u64.
        #[allow(clippy::cast_possible_truncation)]
        Ok(ticks as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn zero_duration_yields_zero_ticks() {
        let duration = Duration::from_nanos(0);
        assert_eq!(UUID::duration_to_ticks(duration), Ok(0));
    }

    #[test]
    fn duration_less_than_one_tick_truncates_to_zero() {
        // A duration of 99 ns should produce 0 ticks, as it's less than one
        // full 100-ns interval.
        let duration = Duration::from_nanos(99);
        assert_eq!(UUID::duration_to_ticks(duration), Ok(0));
    }

    #[test]
    fn duration_of_exactly_one_tick() {
        let duration = Duration::from_nanos(100);
        assert_eq!(UUID::duration_to_ticks(duration), Ok(1));
    }

    #[test]
    fn duration_truncates_to_full_ticks() {
        // A duration of 199 ns should produce 1 tick, verifying that
        // fractional ticks are truncated.
        let duration = Duration::from_nanos(199);
        assert_eq!(UUID::duration_to_ticks(duration), Ok(1));
    }

    #[test]
    fn large_but_valid_duration() {
        // Test with a large duration that is still well within the valid range.
        let ticks = 1u64 << 50;
        let nanos = 100 * ticks;
        let duration = Duration::from_nanos(nanos);
        assert_eq!(UUID::duration_to_ticks(duration), Ok(ticks));
    }

    #[test]
    fn maximum_allowed_duration() {
        // The maximum number of ticks is 2^60 - 1.
        // We must use u128 for the nanosecond calculation to avoid overflow
        // during the test setup itself.
        let max_ticks = (1 << 60) - 1;
        let secs = max_ticks / 10_000_000;
        let nanos = (max_ticks % 10_000_000) * 100;
        let duration = Duration::from_secs(secs) + Duration::from_nanos(nanos);
        assert_eq!(UUID::duration_to_ticks(duration), Ok(max_ticks));
    }

    #[test]
    fn duration_that_overflows() {
        // A duration corresponding to 2^60 ticks should cause an overflow.
        // We must use u128 for the nanosecond calculation.
        let overflow_ticks = 1 << 60;
        let micros = overflow_ticks / 10 + 1;
        let duration = Duration::from_micros(micros);
        assert_eq!(
            UUID::duration_to_ticks(duration),
            Err(DurationToTicksError::TimestampOverflow)
        );
    }
}
