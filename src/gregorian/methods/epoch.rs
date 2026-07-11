use std::time::{SystemTime, UNIX_EPOCH};

use crate::{gregorian::GREGORIAN_OFFSET, Gregorian};

impl Gregorian {
    #[must_use]
    /// Returns the [`Gregorian`] epoch, 1582-10-15 00:00 UTC.
    ///
    /// # Panics
    ///
    /// Panics on platforms whose `SystemTime` cannot represent 1582-10-15,
    /// e.g. Windows, where the clock representation starts at 1601-01-01.
    /// The crate's own time conversions avoid this method and work on all
    /// platforms.
    pub fn epoch() -> SystemTime {
        UNIX_EPOCH - GREGORIAN_OFFSET
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use crate::{gregorian::GREGORIAN_OFFSET, Gregorian};

    #[test]
    fn epoch() {
        let epoch = Gregorian::epoch();

        assert_eq!(
            epoch + GREGORIAN_OFFSET,
            UNIX_EPOCH,
            "Values should be equal."
        );
    }
}
