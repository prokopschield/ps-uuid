use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

use crate::{gregorian::GREGORIAN_OFFSET, Gregorian};

impl Gregorian {
    /// Calculates how much time has elapsed since the [`Gregorian`] epoch
    ///
    /// # Errors
    ///
    /// [`SystemTimeError`] is returned if the current system time is before the [`Gregorian`] epoch.
    ///
    /// # Panics
    ///
    /// Panics if shifting the current time forward by the epoch offset
    /// overflows the platform's clock representation, which requires a clock
    /// within about 387 years of the platform maximum.
    pub fn elapsed() -> Result<Duration, SystemTimeError> {
        // Shift the reading forward by the epoch offset instead of comparing
        // against a materialized 1582-10-15 instant, which platforms with an
        // unsigned clock representation (e.g. Windows) cannot represent.
        let shifted = SystemTime::now()
            .checked_add(GREGORIAN_OFFSET)
            .expect("the system clock exceeds the representable range");

        shifted.duration_since(UNIX_EPOCH)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use crate::Gregorian;

    #[test]
    fn elapsed() {
        let epoch = Gregorian::epoch();

        let elapsed = Gregorian::elapsed()
            .expect("The current system time should be after the Gregorian epoch.");

        let difference = (epoch + elapsed)
            .elapsed()
            .expect("Time should have elapsed since the calculated timestamp");

        assert_eq!(difference.as_secs(), 0, "Values should be equal.");
    }
}
