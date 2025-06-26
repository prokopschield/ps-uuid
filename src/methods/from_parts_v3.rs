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
#[allow(clippy::cast_possible_truncation)]
#[cfg(test)]
mod tests {
    use crate::UUID;

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
}
