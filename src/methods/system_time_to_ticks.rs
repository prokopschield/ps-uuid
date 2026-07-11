use std::time::{SystemTime, UNIX_EPOCH};

use crate::{gregorian::GREGORIAN_OFFSET, UuidConstructionError, UUID};

impl UUID {
    /// Converts a `SystemTime` into an RFC 4122 timestamp (ticks).
    ///
    /// # Arguments
    ///
    /// * `time` - The `SystemTime` to convert.
    ///
    /// # Returns
    ///
    /// The number of 100-nanosecond intervals between the Gregorian epoch
    /// (1582-10-15) and the provided `time`.
    ///
    /// # Errors
    ///
    /// - [`UuidConstructionError::TimestampBeforeEpoch`] if `time` is before
    ///   the Gregorian epoch.
    /// - [`UuidConstructionError::TimestampOverflow`] if `time` is so far in
    ///   the future that the tick count exceeds \( 2^{60} - 1 \).
    pub fn system_time_to_ticks(time: SystemTime) -> Result<u64, UuidConstructionError> {
        // Shift the reading forward by the epoch offset instead of comparing
        // against a materialized 1582-10-15 instant, which platforms with an
        // unsigned clock representation (e.g. Windows, whose clock starts at
        // 1601-01-01) cannot represent. A reading the shift overflows lies
        // within about 387 years of the platform maximum, which every std
        // target places far beyond the 60-bit tick range, so
        // `TimestampOverflow` is the accurate error.
        let Some(shifted) = time.checked_add(GREGORIAN_OFFSET) else {
            return Err(UuidConstructionError::TimestampOverflow);
        };

        // The shifted reading precedes 1970-01-01 exactly when the original
        // reading precedes the Gregorian epoch.
        let Ok(duration_since_epoch) = shifted.duration_since(UNIX_EPOCH) else {
            return Err(UuidConstructionError::TimestampBeforeEpoch);
        };

        // Convert the duration to ticks. The `?` operator will handle a
        // potential `DurationToTicksError` and convert it into a
        // `UuidConstructionError` via the `From` trait implementation.
        let ticks = Self::duration_to_ticks(duration_since_epoch)?;

        Ok(ticks)
    }
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::expect_used)]
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::DurationToTicksError;

    use super::*;

    // --- Tests for system_time_to_ticks ---

    #[test]
    fn time_now_is_valid() {
        // A basic sanity check with the current time.
        let now = SystemTime::now();
        let result = UUID::system_time_to_ticks(now);
        assert!(result.is_ok());
    }

    // The test instant precedes 1601-01-01, which only platforms with a
    // signed clock representation can express.
    #[cfg(unix)]
    #[test]
    fn time_before_gregorian_epoch_fails() {
        // A time one second before the epoch should fail.
        let before_epoch = crate::Gregorian::epoch() - Duration::from_secs(1);
        assert_eq!(
            UUID::system_time_to_ticks(before_epoch),
            Err(UuidConstructionError::TimestampBeforeEpoch)
        );
    }

    // The epoch instant itself is expressible only on platforms with a
    // signed clock representation.
    #[cfg(unix)]
    #[test]
    fn time_at_gregorian_epoch_is_zero_ticks() {
        // The epoch itself should result in zero ticks.
        assert_eq!(UUID::system_time_to_ticks(crate::Gregorian::epoch()), Ok(0));
    }

    #[test]
    fn time_far_in_future_causes_overflow() {
        // A time that would generate 2^60 ticks, causing an overflow.
        // The nanosecond calculation must use u128 to avoid overflowing here.
        let overflow_nanos = (1u128 << 60) * 100;
        let overflow_duration = Duration::new(
            (overflow_nanos / 1_000_000_000) as u64,
            (overflow_nanos % 1_000_000_000) as u32,
        );
        let future_time = UNIX_EPOCH
            + overflow_duration
                .checked_sub(GREGORIAN_OFFSET)
                .expect("subtraction should succeed");

        assert_eq!(
            UUID::system_time_to_ticks(future_time),
            Err(UuidConstructionError::TimestampOverflow)
        );
    }

    // --- Tests for duration_to_ticks (from previous example) ---

    #[test]
    fn zero_duration_yields_zero_ticks() {
        let duration = Duration::from_nanos(0);
        assert_eq!(UUID::duration_to_ticks(duration), Ok(0));
    }

    #[test]
    fn maximum_allowed_duration() {
        let max_ticks = (1u128 << 60) - 1;
        let nanos = max_ticks * 100;
        let duration = Duration::new(
            (nanos / 1_000_000_000) as u64,
            (nanos % 1_000_000_000) as u32,
        );
        assert_eq!(UUID::duration_to_ticks(duration), Ok(max_ticks as u64));
    }

    #[test]
    fn duration_that_overflows() {
        let overflow_ticks = 1u128 << 60;
        let nanos = overflow_ticks * 100;
        let duration = Duration::new(
            (nanos / 1_000_000_000) as u64,
            (nanos % 1_000_000_000) as u32,
        );
        assert_eq!(
            UUID::duration_to_ticks(duration),
            Err(DurationToTicksError::TimestampOverflow)
        );
    }
}
