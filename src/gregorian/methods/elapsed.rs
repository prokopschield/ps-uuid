use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

use crate::{gregorian::GREGORIAN_OFFSET, Gregorian};

impl Gregorian {
    /// Calculates how much time has elapsed since the [`Gregorian`] epoch
    ///
    /// # Errors
    ///
    /// [`SystemTimeError`] is returned if the current system time is before the [`Gregorian`] epoch.
    pub fn elapsed() -> Result<Duration, SystemTimeError> {
        // Shift the reading forward by the epoch offset instead of comparing
        // against a materialized 1582-10-15 instant, which platforms with an
        // unsigned clock representation (e.g. Windows) cannot represent.
        let now = SystemTime::now();

        now.checked_add(GREGORIAN_OFFSET).map_or_else(
            // The shift overflows only when the clock is within the offset of
            // the platform maximum; such a clock is far past the Unix epoch,
            // so the offset can be added in `Duration` space instead.
            || now.duration_since(UNIX_EPOCH).map(|d| d + GREGORIAN_OFFSET),
            |shifted| shifted.duration_since(UNIX_EPOCH),
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use std::time::{Duration, UNIX_EPOCH};

    use crate::{gregorian::GREGORIAN_OFFSET, Gregorian};

    #[test]
    fn elapsed() {
        let since_gregorian = Gregorian::elapsed()
            .expect("The current system time should be after the Gregorian epoch.");

        let since_unix = UNIX_EPOCH
            .elapsed()
            .expect("The current system time should be after the Unix epoch.");

        // The second reading is taken later, so the difference of the two
        // falls short of the epoch offset by exactly the time between the
        // readings.
        let skew = GREGORIAN_OFFSET - (since_gregorian - since_unix);

        assert!(
            skew < Duration::from_secs(1),
            "The readings should differ by the epoch offset."
        );
    }
}
