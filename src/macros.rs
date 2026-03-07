//! Compile-time UUID parsing macro.

/// Parse a UUID from a string literal at compile time.
///
/// Accepts hyphenated, simple, braced, and URN formats.
///
/// # Examples
///
/// ```
/// use ps_uuid::{uuid, UUID};
///
/// const DNS_NAMESPACE: UUID = uuid!("6ba7b810-9dad-11d1-80b4-00c04fd430c8");
/// const SIMPLE: UUID = uuid!("550e8400e29b41d4a716446655440000");
/// const BRACED: UUID = uuid!("{550e8400-e29b-41d4-a716-446655440000}");
/// ```
///
/// # Compile-time Errors
///
/// Invalid UUIDs will cause a compile-time error:
///
/// ```compile_fail
/// use ps_uuid::uuid;
/// const BAD: ps_uuid::UUID = uuid!("not-a-uuid");
/// ```
#[macro_export]
macro_rules! uuid {
    ($s:literal) => {{
        const BYTES: [u8; 16] = $crate::UUID::parse_const($s);
        $crate::UUID::from_bytes(BYTES)
    }};
}

use crate::UUID;

impl UUID {
    /// Parse a UUID string at compile time.
    ///
    /// This is a `const fn` used by the [`uuid!`] macro. Prefer using the macro
    /// for better error messages.
    ///
    /// # Panics
    ///
    /// Panics at compile time if the string is not a valid UUID.
    #[must_use]
    pub const fn parse_const(s: &str) -> [u8; 16] {
        let s = s.as_bytes();
        let (start, end) = find_uuid_bounds(s);
        let len = end - start;

        let expect_hyphens = match len {
            32 => false,
            36 => true,
            _ => panic!("UUID string must be 32 or 36 characters"),
        };

        let mut bytes = [0u8; 16];
        let mut byte_idx = 0;
        let mut i = start;
        let mut pos = 0; // position within the UUID portion

        while i < end {
            let c = s[i];

            if c == b'-' {
                assert!(expect_hyphens, "unexpected hyphen in UUID");
                assert!(is_hyphen_position(pos), "hyphen at invalid position");
                i += 1;
                pos += 1;
                continue;
            }

            let high = hex_digit(c);
            let low = hex_digit(s[i + 1]);
            bytes[byte_idx] = (high << 4) | low;
            byte_idx += 1;
            i += 2;
            pos += 2;
        }

        assert!(byte_idx == 16, "UUID must be exactly 16 bytes");

        bytes
    }
}

/// Returns (start, end) indices for the UUID portion of the string.
const fn find_uuid_bounds(s: &[u8]) -> (usize, usize) {
    let mut start = 0;
    let mut end = s.len();

    // Strip "urn:uuid:" prefix (case-insensitive)
    if s.len() >= 9
        && (s[0] == b'u' || s[0] == b'U')
        && (s[1] == b'r' || s[1] == b'R')
        && (s[2] == b'n' || s[2] == b'N')
        && s[3] == b':'
        && (s[4] == b'u' || s[4] == b'U')
        && (s[5] == b'u' || s[5] == b'U')
        && (s[6] == b'i' || s[6] == b'I')
        && (s[7] == b'd' || s[7] == b'D')
        && s[8] == b':'
    {
        start = 9;
    }

    // Strip braces
    if end > start + 1 && s[start] == b'{' {
        assert!(s[end - 1] == b'}', "mismatched braces");
        start += 1;
        end -= 1;
    }

    (start, end)
}

const fn is_hyphen_position(i: usize) -> bool {
    i == 8 || i == 13 || i == 18 || i == 23
}

const fn hex_digit(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("invalid hex digit"),
    }
}

#[cfg(test)]
mod tests {
    use crate::UUID;

    const EXPECTED: &str = "6ba7b810-9dad-11d1-80b4-00c04fd430c8";

    #[test]
    fn parse_hyphenated() {
        const UUID1: UUID = uuid!("6ba7b810-9dad-11d1-80b4-00c04fd430c8");
        assert_eq!(UUID1.to_string(), "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    }

    #[test]
    fn parse_simple() {
        const UUID1: UUID = uuid!("6ba7b8109dad11d180b400c04fd430c8");
        assert_eq!(UUID1.to_string(), "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    }

    #[test]
    fn parse_braced() {
        const UUID1: UUID = uuid!("{6ba7b810-9dad-11d1-80b4-00c04fd430c8}");
        assert_eq!(UUID1.to_string(), "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    }

    #[test]
    fn parse_braced_simple() {
        const UUID1: UUID = uuid!("{6ba7b8109dad11d180b400c04fd430c8}");
        assert_eq!(UUID1.to_string(), "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    }

    #[test]
    fn parse_urn() {
        const UUID1: UUID = uuid!("urn:uuid:6ba7b810-9dad-11d1-80b4-00c04fd430c8");
        assert_eq!(UUID1.to_string(), "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    }

    #[test]
    fn parse_urn_uppercase() {
        const UUID1: UUID = uuid!("URN:UUID:6BA7B810-9DAD-11D1-80B4-00C04FD430C8");
        assert_eq!(UUID1.to_string(), "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    }

    #[test]
    fn parse_max() {
        const MAX: UUID = uuid!("ffffffff-ffff-ffff-ffff-ffffffffffff");
        assert_eq!(MAX, UUID::max());
    }

    // Hyphenated format - lowercase
    #[test]
    fn hyphenated_lower() {
        const U: UUID = uuid!("6ba7b810-9dad-11d1-80b4-00c04fd430c8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Hyphenated format - uppercase
    #[test]
    fn hyphenated_upper() {
        const U: UUID = uuid!("6BA7B810-9DAD-11D1-80B4-00C04FD430C8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Hyphenated format - mixed case
    #[test]
    fn hyphenated_mixed() {
        const U: UUID = uuid!("6Ba7b810-9DaD-11d1-80B4-00c04FD430c8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Simple format - lowercase
    #[test]
    fn simple_lower() {
        const U: UUID = uuid!("6ba7b8109dad11d180b400c04fd430c8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Simple format - uppercase
    #[test]
    fn simple_upper() {
        const U: UUID = uuid!("6BA7B8109DAD11D180B400C04FD430C8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Simple format - mixed case
    #[test]
    fn simple_mixed() {
        const U: UUID = uuid!("6Ba7b8109DaD11d180B400c04FD430c8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Braced hyphenated format - lowercase
    #[test]
    fn braced_hyphenated_lower() {
        const U: UUID = uuid!("{6ba7b810-9dad-11d1-80b4-00c04fd430c8}");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Braced hyphenated format - uppercase
    #[test]
    fn braced_hyphenated_upper() {
        const U: UUID = uuid!("{6BA7B810-9DAD-11D1-80B4-00C04FD430C8}");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Braced hyphenated format - mixed case
    #[test]
    fn braced_hyphenated_mixed() {
        const U: UUID = uuid!("{6Ba7b810-9DaD-11d1-80B4-00c04FD430c8}");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Braced simple format - lowercase
    #[test]
    fn braced_simple_lower() {
        const U: UUID = uuid!("{6ba7b8109dad11d180b400c04fd430c8}");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Braced simple format - uppercase
    #[test]
    fn braced_simple_upper() {
        const U: UUID = uuid!("{6BA7B8109DAD11D180B400C04FD430C8}");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Braced simple format - mixed case
    #[test]
    fn braced_simple_mixed() {
        const U: UUID = uuid!("{6Ba7b8109DaD11d180B400c04FD430c8}");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // URN hyphenated format - lowercase
    #[test]
    fn urn_hyphenated_lower() {
        const U: UUID = uuid!("urn:uuid:6ba7b810-9dad-11d1-80b4-00c04fd430c8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // URN hyphenated format - uppercase
    #[test]
    fn urn_hyphenated_upper() {
        const U: UUID = uuid!("URN:UUID:6BA7B810-9DAD-11D1-80B4-00C04FD430C8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // URN hyphenated format - mixed case (prefix and UUID)
    #[test]
    fn urn_hyphenated_mixed() {
        const U: UUID = uuid!("Urn:UuId:6Ba7b810-9DaD-11d1-80B4-00c04FD430c8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // URN simple format - lowercase
    #[test]
    fn urn_simple_lower() {
        const U: UUID = uuid!("urn:uuid:6ba7b8109dad11d180b400c04fd430c8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // URN simple format - uppercase
    #[test]
    fn urn_simple_upper() {
        const U: UUID = uuid!("URN:UUID:6BA7B8109DAD11D180B400C04FD430C8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // URN simple format - mixed case
    #[test]
    fn urn_simple_mixed() {
        const U: UUID = uuid!("Urn:UuId:6Ba7b8109DaD11d180B400c04FD430c8");
        assert_eq!(U.to_string(), EXPECTED);
    }

    // Special values
    #[test]
    fn parse_nil() {
        const NIL: UUID = uuid!("00000000-0000-0000-0000-000000000000");
        assert_eq!(NIL, UUID::nil());
    }

    #[test]
    fn parse_max_lower() {
        const MAX: UUID = uuid!("ffffffff-ffff-ffff-ffff-ffffffffffff");
        assert_eq!(MAX, UUID::max());
    }

    #[test]
    fn parse_max_upper() {
        const MAX: UUID = uuid!("FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF");
        assert_eq!(MAX, UUID::max());
    }

    #[test]
    fn usable_in_const_context() {
        const DNS_NS: UUID = uuid!("6ba7b810-9dad-11d1-80b4-00c04fd430c8");
        static STATIC_UUID: UUID = uuid!("6ba7b811-9dad-11d1-80b4-00c04fd430c8");

        assert!(DNS_NS.is_v1());
        assert!(STATIC_UUID.is_v1());
    }
}
