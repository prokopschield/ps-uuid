//! URN formatting for UUID.

use core::fmt;

use crate::UUID;

/// A UUID formatted as a Uniform Resource Name.
///
/// Created by calling [`UUID::urn()`], this wrapper implements [`Display`]
/// to render the UUID with the `urn:uuid:` prefix as specified in RFC 4122.
///
/// ```text
/// urn:uuid:550e8400-e29b-41d4-a716-446655440000
/// ```
///
/// [`Display`]: core::fmt::Display
#[derive(Clone, Copy, Debug)]
pub struct Urn(UUID);

impl fmt::Display for Urn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = &self.0.bytes;
        write!(
            f,
            "urn:uuid:{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0], b[1], b[2], b[3],
            b[4], b[5],
            b[6], b[7],
            b[8], b[9],
            b[10], b[11], b[12], b[13], b[14], b[15]
        )
    }
}

impl From<Urn> for UUID {
    #[inline]
    fn from(urn: Urn) -> Self {
        urn.0
    }
}

impl UUID {
    /// Returns a formatter for the URN format.
    ///
    /// # Example
    ///
    /// ```
    /// use ps_uuid::UUID;
    ///
    /// let uuid = UUID::nil();
    /// assert_eq!(uuid.urn().to_string(), "urn:uuid:00000000-0000-0000-0000-000000000000");
    /// ```
    #[inline]
    #[must_use]
    pub const fn urn(self) -> Urn {
        Urn(self)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use crate::UUID;

    #[test]
    fn nil_formats_correctly() {
        assert_eq!(
            UUID::nil().urn().to_string(),
            "urn:uuid:00000000-0000-0000-0000-000000000000"
        );
    }

    #[test]
    fn max_formats_correctly() {
        assert_eq!(
            UUID::max().urn().to_string(),
            "urn:uuid:ffffffff-ffff-ffff-ffff-ffffffffffff"
        );
    }

    #[test]
    fn urn_has_correct_length() {
        let uuid = UUID::gen_v4();
        let urn = uuid.urn().to_string();
        assert_eq!(urn.len(), 45); // "urn:uuid:" (9) + hyphenated (36) = 45
    }

    #[test]
    fn urn_starts_with_prefix() {
        let uuid = UUID::gen_v4();
        let urn = uuid.urn().to_string();
        assert!(urn.starts_with("urn:uuid:"));
    }

    #[test]
    fn urn_contains_hyphenated_uuid() {
        let uuid = UUID::gen_v4();
        let urn = uuid.urn().to_string();
        let hyphenated = uuid.hyphenated().to_string();
        assert!(urn.ends_with(&hyphenated));
    }

    #[test]
    fn urn_is_lowercase() {
        let uuid = UUID::from_bytes([
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56,
            0x78, 0x9A,
        ]);
        let urn = uuid.urn().to_string();
        assert_eq!(urn, "urn:uuid:abcdef12-3456-789a-bcde-f0123456789a");
    }

    #[test]
    fn round_trip_parse() {
        let uuid = UUID::gen_v4();
        let urn = uuid.urn().to_string();
        let parsed: UUID = urn.parse().expect("URN format should parse");
        assert_eq!(parsed, uuid);
    }
}
