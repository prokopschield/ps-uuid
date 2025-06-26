use crate::UUID;

// ────────────────────────────────────────────────────────────────────────────
// v3 constructor from a pre-computed MD5 digest
// ────────────────────────────────────────────────────────────────────────────

impl UUID {
    /// Builds an RFC-4122 Version-3 UUID from a raw 16-byte MD5 digest.
    ///
    /// The caller must supply the digest that results from hashing
    /// `namespace.bytes || name`.  The function overwrites the version and
    /// variant fields via `.with_version(3)` and returns the finished UUID.
    #[must_use]
    pub const fn from_parts_v3(digest: [u8; 16]) -> Self {
        Self { bytes: digest }.with_version(3)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests
// ────────────────────────────────────────────────────────────────────────────
#[allow(clippy::cast_possible_truncation, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{md5, UUID};

    // Helper: RFC-4122 variant check (two MSBs = 10)
    const fn is_rfc4122_variant(b: u8) -> bool {
        (b & 0b1100_0000) == 0b1000_0000
    }

    #[test]
    fn sets_version_and_variant_correctly() {
        for &input in &[[0u8; 16], [0xFFu8; 16]] {
            let uuid = UUID::from_parts_v3(input);
            assert_eq!(uuid.version(), Some(3));
            assert!(is_rfc4122_variant(uuid.bytes[8]));
        }
    }

    #[test]
    fn preserves_all_other_bits() {
        let mut digest = [0u8; 16];

        for (i, item) in digest.iter_mut().enumerate() {
            *item = i as u8;
        }

        let uuid = UUID::from_parts_v3(digest);

        for i in 0..16 {
            match i {
                // Upper nibble of byte 6 carries the version.
                6 => assert_eq!(uuid.bytes[6] & 0x0F, digest[6] & 0x0F),
                // Upper two bits of byte 8 carry the variant.
                8 => assert_eq!(uuid.bytes[8] & 0x3F, digest[8] & 0x3F),
                _ => assert_eq!(uuid.bytes[i], digest[i]),
            }
        }
    }

    #[test]
    fn matches_gen_v3_reference_implementation() {
        // Reference data from RFC 4122, Appendix C:
        // namespace = DNS (6ba7b810-9dad-11d1-80b4-00c04fd430c8)
        // name      = "python.org"
        let ns = UUID::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
        let name = b"python.org";

        // v3 via the public constructor
        let via_api = UUID::new_v3(&ns, name);

        // Compute digest directly and call `from_parts_v3`
        let digest = md5(&[&ns.bytes[..], name].concat());
        let via_parts = UUID::from_parts_v3(digest);

        assert_eq!(via_parts.bytes, via_api.bytes);
    }
}
