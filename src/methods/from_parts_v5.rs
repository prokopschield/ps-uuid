use crate::UUID;

// ────────────────────────────────────────────────────────────────────────────
// v5 constructor from a pre-computed SHA-1 digest
// ────────────────────────────────────────────────────────────────────────────

impl UUID {
    /// Builds an RFC-4122 Version-5 UUID from a raw 20-byte SHA-1 digest.
    ///
    /// The caller must supply the digest that results from hashing
    /// `namespace.bytes || name`.  
    /// The first 16 bytes of the digest are copied into the UUID; the
    /// version and variant fields are then fixed via `.with_version(5)`.
    #[must_use]
    pub fn from_parts_v5<D>(digest: D) -> Self
    where
        D: AsRef<[u8]>,
    {
        let digest = digest.as_ref();
        let mut uuid = Self::nil();

        uuid.bytes[..digest.len().min(16)].copy_from_slice(&digest[..digest.len().min(16)]);

        uuid.with_version(5)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests
// ────────────────────────────────────────────────────────────────────────────
#[allow(clippy::cast_possible_truncation, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{sha1, UUID};

    // Two MSBs must equal `10`.
    const fn is_rfc4122_variant(b: u8) -> bool {
        (b & 0b1100_0000) == 0b1000_0000
    }

    #[test]
    fn sets_version_and_variant_correctly() {
        for &input in &[[0u8; 20], [0xFFu8; 20]] {
            let uuid = UUID::from_parts_v5(input);
            assert_eq!(uuid.get_version(), Some(5));
            assert!(is_rfc4122_variant(uuid.bytes[8]));
        }
    }

    #[test]
    fn preserves_all_other_bits() {
        let mut digest = [0u8; 20];
        for (i, b) in digest.iter_mut().enumerate() {
            *b = i as u8;
        }

        let uuid = UUID::from_parts_v5(digest);

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
    fn matches_rfc_reference_example() {
        // RFC-4122 Appendix C  (DNS namespace, name = "python.org")
        let ns = UUID::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
        let name = b"python.org";

        // Digest and UUID via `from_parts_v5`
        let digest = sha1(&[&ns.bytes[..], name].concat());
        let via_parts = UUID::from_parts_v5(digest);

        // Expected UUID from the RFC
        let expected = UUID::from_str("886313e1-3b8a-5372-9b90-0c9aee199e5d").unwrap();

        assert_eq!(via_parts.bytes, expected.bytes);
    }
}
