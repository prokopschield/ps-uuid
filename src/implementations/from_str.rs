use std::str::FromStr;

use crate::{error::UuidParseError, UUID};

const HYPHEN_POS: [usize; 4] = [8, 13, 18, 23];

impl FromStr for UUID {
    type Err = UuidParseError;

    /// Accept every standard UUID spelling:
    ///   - canonical 36-byte form           `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`
    ///   - 32 hex digits without hyphens    `xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx`
    ///   - surrounded by braces             `{…}`  (either of the above)
    ///   - as an URN                        `urn:uuid:<canonical>`
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        // 1. Strip leading `urn:uuid:` (case-insensitive).
        const URN: &str = "urn:uuid:";
        if s.len() >= URN.len() && s[..URN.len()].eq_ignore_ascii_case(URN) {
            s = &s[URN.len()..];
        }

        // 2. Strip optional surrounding braces.
        if s.starts_with('{') {
            if !s.ends_with('}') {
                return Err(UuidParseError::InvalidBraces);
            }
            s = &s[1..s.len() - 1];
        } else if s.ends_with('}') {
            return Err(UuidParseError::InvalidBraces);
        }

        // 3. Decide expected format.
        let expect_hyphens = match s.len() {
            32 => false,
            36 => true,
            _ => return Err(UuidParseError::InvalidLength),
        };

        // 4. Prepare to collect the 32 hexadecimal nibbles.
        let mut nibbles = [0u8; 32]; // 32 * 4 bit = 128 bit
        let mut nib_i = 0;

        for (idx, ch) in s.chars().enumerate() {
            if ch == '-' {
                // Hyphens allowed only in the canonical positions.
                if !expect_hyphens || !HYPHEN_POS.contains(&idx) {
                    return Err(UuidParseError::InvalidHyphenPlacement);
                }
                continue;
            }

            // Convert ASCII hex → value.
            let val = match ch {
                '0'..='9' => ch as u8 - b'0',
                'a'..='f' => ch as u8 - b'a' + 10,
                'A'..='F' => ch as u8 - b'A' + 10,
                _ => return Err(UuidParseError::InvalidCharacter { ch, idx }),
            };
            if nib_i >= 32 {
                return Err(UuidParseError::InvalidLength);
            }
            nibbles[nib_i] = val;
            nib_i += 1;
        }

        if nib_i != 32 {
            return Err(UuidParseError::InvalidLength);
        }

        // 5. Pack nibbles into 16 bytes.
        let mut bytes = [0u8; 16];
        for i in 0..16 {
            bytes[i] = (nibbles[2 * i] << 4) | nibbles[2 * i + 1];
        }

        Ok(Self { bytes })
    }
}

#[allow(clippy::expect_used, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    // Same sample used by RFC 4122.
    const RFC_SAMPLE_CANON: &str = "6ba7b810-9dad-11d1-80b4-00c04fd430c8";
    const RFC_SAMPLE_BYTES: [u8; 16] = [
        0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ];

    // ---------------------------------------------------------------------
    // Happy-path cases
    // ---------------------------------------------------------------------

    #[test]
    fn parses_all_standard_encodings() {
        let variants = [
            // canonical
            RFC_SAMPLE_CANON,
            // no hyphens
            "6ba7b8109dad11d180b400c04fd430c8",
            // uppercase
            "6BA7B810-9DAD-11D1-80B4-00C04FD430C8",
            // braces
            "{6ba7b810-9dad-11d1-80b4-00c04fd430c8}",
            // braces without hyphens
            "{6ba7b8109dad11d180b400c04fd430c8}",
            // URN
            "urn:uuid:6ba7b810-9dad-11d1-80b4-00c04fd430c8",
            // URN with braces
            "URN:UUID:{6BA7B810-9DAD-11D1-80B4-00C04FD430C8}",
        ];

        for s in variants {
            let uuid = UUID::from_str(s).expect("must parse");
            assert_eq!(
                uuid.bytes, RFC_SAMPLE_BYTES,
                "parsing failed for variant: {s}"
            );
        }
    }

    // ---------------------------------------------------------------------
    // Error cases
    // ---------------------------------------------------------------------

    #[test]
    fn rejects_wrong_length() {
        assert_eq!(
            UUID::from_str("123456").unwrap_err(),
            UuidParseError::InvalidLength
        );
    }

    #[test]
    fn rejects_invalid_hex() {
        let bad = "6ba7b810-9dad-11d1-80b4-00c04fd430cg"; // 'g'
        match UUID::from_str(bad) {
            Err(UuidParseError::InvalidCharacter { ch: 'g', idx }) => assert_eq!(idx, 35),
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn rejects_bad_hyphen_positions() {
        let bad = "6ba7b810-9dad11d1-80b4-00c04fd430c8"; // hyphen missing at 18

        assert_eq!(UUID::from_str(bad), Err(UuidParseError::InvalidLength));
    }

    // ---------------------------------------------------------------------
    // Round-trip sanity
    // ---------------------------------------------------------------------

    #[test]
    fn round_trip_hyphenated() {
        let uuid = UUID::from_str(RFC_SAMPLE_CANON).unwrap();
        // Assuming you have a `to_hyphenated_string()` or `Display` impl.
        let s = format!("{uuid}");
        let again = UUID::from_str(&s).unwrap();
        assert_eq!(uuid.bytes, again.bytes);
    }

    // ---------------------------------------------------------------------
    // Happy-path: all standard encodings
    // ---------------------------------------------------------------------

    #[test]
    fn parses_canonical() {
        let uuid = UUID::from_str(RFC_SAMPLE_CANON).unwrap();
        assert_eq!(uuid.bytes, RFC_SAMPLE_BYTES);
    }

    #[test]
    fn parses_no_hyphens() {
        let uuid = UUID::from_str("6ba7b8109dad11d180b400c04fd430c8").unwrap();
        assert_eq!(uuid.bytes, RFC_SAMPLE_BYTES);
    }

    #[test]
    fn parses_uppercase() {
        let uuid = UUID::from_str("6BA7B810-9DAD-11D1-80B4-00C04FD430C8").unwrap();
        assert_eq!(uuid.bytes, RFC_SAMPLE_BYTES);
    }

    #[test]
    fn parses_braces_canonical() {
        let uuid = UUID::from_str("{6ba7b810-9dad-11d1-80b4-00c04fd430c8}").unwrap();
        assert_eq!(uuid.bytes, RFC_SAMPLE_BYTES);
    }

    #[test]
    fn parses_braces_no_hyphens() {
        let uuid = UUID::from_str("{6ba7b8109dad11d180b400c04fd430c8}").unwrap();
        assert_eq!(uuid.bytes, RFC_SAMPLE_BYTES);
    }

    #[test]
    fn parses_urn_canonical() {
        let uuid = UUID::from_str("urn:uuid:6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
        assert_eq!(uuid.bytes, RFC_SAMPLE_BYTES);
    }

    #[test]
    fn parses_urn_braces() {
        let uuid = UUID::from_str("urn:uuid:{6ba7b810-9dad-11d1-80b4-00c04fd430c8}").unwrap();
        assert_eq!(uuid.bytes, RFC_SAMPLE_BYTES);
    }

    #[test]
    fn parses_urn_uppercase() {
        let uuid = UUID::from_str("URN:UUID:6BA7B810-9DAD-11D1-80B4-00C04FD430C8").unwrap();
        assert_eq!(uuid.bytes, RFC_SAMPLE_BYTES);
    }

    #[test]
    fn parses_urn_braces_uppercase() {
        let uuid = UUID::from_str("URN:UUID:{6BA7B810-9DAD-11D1-80B4-00C04FD430C8}").unwrap();
        assert_eq!(uuid.bytes, RFC_SAMPLE_BYTES);
    }

    // ---------------------------------------------------------------------
    // Edge cases: whitespace, empty, minimal/maximal values
    // ---------------------------------------------------------------------

    #[test]
    fn rejects_leading_trailing_whitespace() {
        assert_eq!(
            UUID::from_str(" 6ba7b810-9dad-11d1-80b4-00c04fd430c8"),
            Err(UuidParseError::InvalidLength)
        );
        assert_eq!(
            UUID::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8 "),
            Err(UuidParseError::InvalidLength)
        );
    }

    #[test]
    fn rejects_empty_string() {
        assert_eq!(
            UUID::from_str("").unwrap_err(),
            UuidParseError::InvalidLength
        );
    }

    #[test]
    fn parses_all_zero_uuid() {
        let uuid = UUID::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        assert_eq!(uuid.bytes, [0u8; 16]);
    }

    #[test]
    fn parses_all_ff_uuid() {
        let uuid = UUID::from_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap();
        assert_eq!(uuid.bytes, [0xFFu8; 16]);
    }

    // ---------------------------------------------------------------------
    // Error cases: length, hyphens, braces, invalid chars, overflow
    // ---------------------------------------------------------------------

    #[test]
    fn rejects_too_short() {
        assert_eq!(
            UUID::from_str("1234").unwrap_err(),
            UuidParseError::InvalidLength
        );
    }

    #[test]
    fn rejects_too_long() {
        let s = format!("{RFC_SAMPLE_CANON}00");
        assert_eq!(
            UUID::from_str(&s).unwrap_err(),
            UuidParseError::InvalidLength
        );
    }

    #[test]
    fn rejects_missing_hyphens_in_canonical() {
        let s = "6ba7b8109dad-11d1-80b4-00c04fd430c8";
        assert_eq!(
            UUID::from_str(s).unwrap_err(),
            UuidParseError::InvalidLength
        );
    }

    #[test]
    fn rejects_extra_hyphens() {
        let s = "6ba7b810--9dad-11d1-80b4-00c04fd430c8";
        assert_eq!(UUID::from_str(s), Err(UuidParseError::InvalidLength));
    }

    #[test]
    fn rejects_hyphens_in_no_hyphen_form() {
        let s = "6ba7b8109dad11d1-80b4-00c04fd430c8";
        assert_eq!(
            UUID::from_str(s).unwrap_err(),
            UuidParseError::InvalidLength // because length is not 32 or 36
        );
    }

    #[test]
    fn rejects_invalid_hex_digit() {
        let mut bad = RFC_SAMPLE_CANON.to_string();
        bad.replace_range(0..1, "G"); // 'G' is not a hex digit
        assert_eq!(
            UUID::from_str(&bad).unwrap_err(),
            UuidParseError::InvalidCharacter { ch: 'G', idx: 0 }
        );
    }

    #[test]
    fn rejects_invalid_hex_digit_in_no_hyphen() {
        let mut bad = "6ba7b8109dad11d180b400c04fd430c8".to_string();
        bad.replace_range(31..32, "Z");
        assert_eq!(
            UUID::from_str(&bad).unwrap_err(),
            UuidParseError::InvalidCharacter { ch: 'Z', idx: 31 }
        );
    }

    #[test]
    fn rejects_mismatched_braces() {
        assert_eq!(
            UUID::from_str("{6ba7b810-9dad-11d1-80b4-00c04fd430c8"),
            Err(UuidParseError::InvalidBraces)
        );
        assert_eq!(
            UUID::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8}"),
            Err(UuidParseError::InvalidBraces)
        );
        assert_eq!(
            UUID::from_str("{6ba7b810-9dad-11d1-80b4-00c04fd430c8}}"),
            Err(UuidParseError::InvalidLength)
        );
    }

    #[test]
    fn rejects_double_braces() {
        assert_eq!(
            UUID::from_str("{{6ba7b810-9dad-11d1-80b4-00c04fd430c8}}"),
            Err(UuidParseError::InvalidLength)
        );
    }

    #[test]
    fn rejects_urn_with_invalid_uuid() {
        let s = "urn:uuid:6ba7b810-9dad-11d1-80b4-00c04fd4308Z";
        assert_eq!(
            UUID::from_str(s).unwrap_err(),
            UuidParseError::InvalidCharacter { ch: 'Z', idx: 35 }
        );
    }

    #[test]
    fn rejects_urn_with_braces_and_invalid_uuid() {
        let s = "urn:uuid:{6ba7b810-9dad-11d1-80b4-00c04fd430cZ}";
        assert_eq!(
            UUID::from_str(s).unwrap_err(),
            UuidParseError::InvalidCharacter { ch: 'Z', idx: 35 }
        );
    }

    #[test]
    fn rejects_urn_with_mismatched_braces() {
        let s = "urn:uuid:{6ba7b810-9dad-11d1-80b4-00c04fd430c8";
        assert_eq!(
            UUID::from_str(s).unwrap_err(),
            UuidParseError::InvalidBraces
        );
    }

    #[test]
    fn rejects_urn_with_extra_characters() {
        let s = "urn:uuid:6ba7b810-9dad-11d1-80b4-00c04fd430c8extra";
        assert_eq!(
            UUID::from_str(s).unwrap_err(),
            UuidParseError::InvalidLength
        );
    }

    // ---------------------------------------------------------------------
    // Pathological: all hyphens, all braces, all colons, etc.
    // ---------------------------------------------------------------------

    #[test]
    fn rejects_all_hyphens() {
        let s = "------------------------------------";
        assert_eq!(
            UUID::from_str(s),
            Err(UuidParseError::InvalidHyphenPlacement)
        );
    }

    #[test]
    fn rejects_all_braces() {
        let s = "{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{";
        assert_eq!(UUID::from_str(s), Err(UuidParseError::InvalidBraces));
    }

    #[test]
    fn rejects_all_colons() {
        let s = "::::::::::::::::::::::::::::::::::::";
        assert_eq!(
            UUID::from_str(s).unwrap_err(),
            UuidParseError::InvalidCharacter { ch: ':', idx: 0 }
        );
    }

    // ---------------------------------------------------------------------
    // Round-trip and case-insensitivity
    // ---------------------------------------------------------------------

    #[test]
    fn round_trip_canonical() {
        let uuid = UUID::from_str(RFC_SAMPLE_CANON).unwrap();
        let s = format!("{uuid}");
        let again = UUID::from_str(&s).unwrap();
        assert_eq!(uuid.bytes, again.bytes);
    }

    #[test]
    fn accepts_mixed_case() {
        let s = "6Ba7B810-9dAD-11D1-80b4-00C04fD430C8";
        let uuid = UUID::from_str(s).unwrap();
        assert_eq!(uuid.bytes, RFC_SAMPLE_BYTES);
    }

    #[test]
    fn accepts_urn_with_mixed_case_prefix() {
        let s = "UrN:UuId:6ba7b810-9dad-11d1-80b4-00c04fd430c8";
        let uuid = UUID::from_str(s).unwrap();
        assert_eq!(uuid.bytes, RFC_SAMPLE_BYTES);
    }
}
