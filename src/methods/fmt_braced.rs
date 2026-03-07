//! Braced formatting for UUID.

use core::fmt;

use crate::UUID;

/// A UUID formatted with surrounding braces.
///
/// Created by calling [`UUID::braced()`], this wrapper implements [`Display`]
/// to render the UUID enclosed in curly braces, a format commonly used by
/// Microsoft technologies.
///
/// ```text
/// {550e8400-e29b-41d4-a716-446655440000}
/// ```
///
/// [`Display`]: core::fmt::Display
#[derive(Clone, Copy, Debug)]
pub struct Braced(UUID);

impl fmt::Display for Braced {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = &self.0.bytes;
        write!(
            f,
            "{{{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}",
            b[0], b[1], b[2], b[3],
            b[4], b[5],
            b[6], b[7],
            b[8], b[9],
            b[10], b[11], b[12], b[13], b[14], b[15]
        )
    }
}

impl From<Braced> for UUID {
    #[inline]
    fn from(braced: Braced) -> Self {
        braced.0
    }
}

impl UUID {
    /// Returns a formatter for the braced format.
    ///
    /// # Example
    ///
    /// ```
    /// use ps_uuid::UUID;
    ///
    /// let uuid = UUID::nil();
    /// assert_eq!(uuid.braced().to_string(), "{00000000-0000-0000-0000-000000000000}");
    /// ```
    #[inline]
    #[must_use]
    pub const fn braced(self) -> Braced {
        Braced(self)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use crate::UUID;

    #[test]
    fn nil_formats_correctly() {
        assert_eq!(
            UUID::nil().braced().to_string(),
            "{00000000-0000-0000-0000-000000000000}"
        );
    }

    #[test]
    fn max_formats_correctly() {
        assert_eq!(
            UUID::max().braced().to_string(),
            "{ffffffff-ffff-ffff-ffff-ffffffffffff}"
        );
    }

    #[test]
    fn braced_has_correct_length() {
        let uuid = UUID::gen_v4();
        let braced = uuid.braced().to_string();
        assert_eq!(braced.len(), 38); // "{" (1) + hyphenated (36) + "}" (1) = 38
    }

    #[test]
    fn braced_starts_and_ends_with_braces() {
        let uuid = UUID::gen_v4();
        let braced = uuid.braced().to_string();
        assert!(braced.starts_with('{'));
        assert!(braced.ends_with('}'));
    }

    #[test]
    fn braced_contains_hyphenated_uuid() {
        let uuid = UUID::gen_v4();
        let braced = uuid.braced().to_string();
        let hyphenated = uuid.hyphenated().to_string();
        assert_eq!(&braced[1..37], hyphenated);
    }

    #[test]
    fn braced_is_lowercase() {
        let uuid = UUID::from_bytes([
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56,
            0x78, 0x9A,
        ]);
        let braced = uuid.braced().to_string();
        assert_eq!(braced, "{abcdef12-3456-789a-bcde-f0123456789a}");
    }

    #[test]
    fn round_trip_parse() {
        let uuid = UUID::gen_v4();
        let braced = uuid.braced().to_string();
        let parsed: UUID = braced.parse().expect("braced format should parse");
        assert_eq!(parsed, uuid);
    }
}
