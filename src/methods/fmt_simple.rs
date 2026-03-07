//! Simple (non-hyphenated) formatting for UUID.

use core::fmt;

use crate::UUID;

/// A UUID formatted as a simple, unadorned sequence of 32 hexadecimal digits.
///
/// Created by calling [`UUID::simple()`], this wrapper implements [`Display`]
/// to render the UUID without hyphens or any other separators.
///
/// ```text
/// 550e8400e29b41d4a716446655440000
/// ```
///
/// [`Display`]: core::fmt::Display
#[derive(Clone, Copy, Debug)]
pub struct Simple(UUID);

impl fmt::Display for Simple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = &self.0.bytes;
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
            b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
        )
    }
}

impl From<Simple> for UUID {
    #[inline]
    fn from(simple: Simple) -> Self {
        simple.0
    }
}

impl UUID {
    /// Returns a formatter for the simple (non-hyphenated) format.
    ///
    /// # Example
    ///
    /// ```
    /// use ps_uuid::UUID;
    ///
    /// let uuid = UUID::nil();
    /// assert_eq!(uuid.simple().to_string(), "00000000000000000000000000000000");
    /// ```
    #[inline]
    #[must_use]
    pub const fn simple(self) -> Simple {
        Simple(self)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use crate::UUID;

    #[test]
    fn nil_formats_as_32_zeros() {
        assert_eq!(
            UUID::nil().simple().to_string(),
            "00000000000000000000000000000000"
        );
    }

    #[test]
    fn max_formats_as_32_fs() {
        assert_eq!(
            UUID::max().simple().to_string(),
            "ffffffffffffffffffffffffffffffff"
        );
    }

    #[test]
    fn simple_has_no_hyphens() {
        let uuid = UUID::gen_v4();
        let simple = uuid.simple().to_string();
        assert_eq!(simple.len(), 32);
        assert!(!simple.contains('-'));
    }

    #[test]
    fn simple_matches_display_without_hyphens() {
        let uuid = UUID::gen_v4();
        let display = uuid.to_string();
        let simple = uuid.simple().to_string();
        assert_eq!(simple, display.replace('-', ""));
    }

    #[test]
    fn simple_is_lowercase_hex() {
        let uuid = UUID::from_bytes([
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56,
            0x78, 0x9A,
        ]);
        let simple = uuid.simple().to_string();
        assert_eq!(simple, "abcdef123456789abcdef0123456789a");
    }

    #[test]
    fn round_trip_parse() {
        let uuid = UUID::gen_v4();
        let simple = uuid.simple().to_string();
        let parsed: UUID = simple.parse().expect("simple format should parse");
        assert_eq!(parsed, uuid);
    }
}
