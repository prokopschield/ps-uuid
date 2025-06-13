use std::time::{Duration, SystemTime, SystemTimeError};

use crate::Gregorian;

impl Gregorian {
    /// Calculates how much time has elapsed since the [`Gregorian`] epoch
    ///
    /// # Errors
    ///
    /// [`SystemTimeError`] is returned if the current system time is before the [`Gregorian`] epoch.
    pub fn elapsed() -> Result<Duration, SystemTimeError> {
        SystemTime::now().duration_since(Self::epoch())
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
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
