//! Hyphenated formatting for UUID.

use core::fmt;

use crate::UUID;

/// A UUID formatted in the canonical hyphenated representation.
///
/// Created by calling [`UUID::hyphenated()`], this wrapper implements [`Display`]
/// to render the UUID in the standard 8-4-4-4-12 format defined by RFC 4122.
///
/// ```text
/// 550e8400-e29b-41d4-a716-446655440000
/// ```
///
/// [`Display`]: core::fmt::Display
#[derive(Clone, Copy, Debug)]
pub struct Hyphenated(UUID);

impl fmt::Display for Hyphenated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = &self.0.bytes;
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0], b[1], b[2], b[3],
            b[4], b[5],
            b[6], b[7],
            b[8], b[9],
            b[10], b[11], b[12], b[13], b[14], b[15]
        )
    }
}

impl From<Hyphenated> for UUID {
    #[inline]
    fn from(hyphenated: Hyphenated) -> Self {
        hyphenated.0
    }
}

impl UUID {
    /// Returns a formatter for the hyphenated (standard) format.
    ///
    /// This produces the same output as the `Display` implementation.
    ///
    /// # Example
    ///
    /// ```
    /// use ps_uuid::UUID;
    ///
    /// let uuid = UUID::nil();
    /// assert_eq!(uuid.hyphenated().to_string(), "00000000-0000-0000-0000-000000000000");
    /// ```
    #[inline]
    #[must_use]
    pub const fn hyphenated(self) -> Hyphenated {
        Hyphenated(self)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use crate::UUID;

    #[test]
    fn nil_formats_correctly() {
        assert_eq!(
            UUID::nil().hyphenated().to_string(),
            "00000000-0000-0000-0000-000000000000"
        );
    }

    #[test]
    fn max_formats_correctly() {
        assert_eq!(
            UUID::max().hyphenated().to_string(),
            "ffffffff-ffff-ffff-ffff-ffffffffffff"
        );
    }

    #[test]
    fn hyphenated_has_correct_length() {
        let uuid = UUID::gen_v4();
        let hyphenated = uuid.hyphenated().to_string();
        assert_eq!(hyphenated.len(), 36);
    }

    #[test]
    fn hyphenated_has_hyphens_at_correct_positions() {
        let uuid = UUID::gen_v4();
        let hyphenated = uuid.hyphenated().to_string();
        let chars: Vec<char> = hyphenated.chars().collect();
        assert_eq!(chars[8], '-');
        assert_eq!(chars[13], '-');
        assert_eq!(chars[18], '-');
        assert_eq!(chars[23], '-');
    }

    #[test]
    fn hyphenated_matches_display() {
        let uuid = UUID::gen_v4();
        assert_eq!(uuid.hyphenated().to_string(), uuid.to_string());
    }

    #[test]
    fn hyphenated_is_lowercase_hex() {
        let uuid = UUID::from_bytes([
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56,
            0x78, 0x9A,
        ]);
        let hyphenated = uuid.hyphenated().to_string();
        assert_eq!(hyphenated, "abcdef12-3456-789a-bcde-f0123456789a");
    }

    #[test]
    fn round_trip_parse() {
        let uuid = UUID::gen_v4();
        let hyphenated = uuid.hyphenated().to_string();
        let parsed: UUID = hyphenated.parse().expect("hyphenated format should parse");
        assert_eq!(parsed, uuid);
    }
}
