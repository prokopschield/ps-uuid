use crate::{Md5, UUID};

impl UUID {
    /// Builds an RFC-4122 Version-3 UUID from `namespace || name`.
    ///
    /// The function
    ///   1. hashes the concatenation `namespace.bytes || name` with MD5,
    ///   2. calls `from_parts_v3` to set version = 3 + RFC-4122 variant,
    ///   3. returns the finished UUID.
    #[must_use]
    pub fn new_v3<N>(namespace: &Self, name: N) -> Self
    where
        N: AsRef<[u8]>,
    {
        let mut hasher = Md5::new();

        hasher.update(namespace.as_bytes());
        hasher.update(name.as_ref());

        let digest = hasher.finalize();

        Self::from_parts_v3(digest)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test-suite
// ────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use crate::UUID;

    use super::*;

    // Helper: two MSBs must be `10`
    const fn is_rfc4122_variant(b: u8) -> bool {
        (b & 0b1100_0000) == 0b1000_0000
    }

    #[test]
    fn version_and_variant_are_correct() {
        let ns = UUID { bytes: [0u8; 16] };
        let uuid = UUID::new_v3(&ns, "abc");
        assert_eq!(uuid.get_version(), Some(3));
        assert!(is_rfc4122_variant(uuid.bytes[8]));
    }

    #[test]
    fn identical_inputs_yield_identical_uuids() {
        let ns = UUID {
            bytes: [0x42u8; 16],
        };
        let name = "same-name";
        assert_eq!(
            UUID::new_v3(&ns, name).bytes,
            UUID::new_v3(&ns, name).bytes,
            "Deterministic output expected"
        );
    }

    #[test]
    fn matches_manual_digest() {
        let ns = UUID { bytes: [1u8; 16] };
        let name = b"xyz";

        // via API
        let via_api = UUID::new_v3(&ns, name);

        // manual digest  →  from_parts_v3
        let mut h = Md5::new();
        h.update(ns.as_bytes());
        h.update(name);
        let digest = h.finalize();
        let mut dig = [0u8; 16];
        dig.copy_from_slice(&digest);
        let via_parts = UUID::from_parts_v3(dig);

        assert_eq!(via_api.bytes, via_parts.bytes);
    }

    #[test]
    fn rfc_example_python_org() {
        // RFC-4122 Appendix C (DNS namespace + "python.org")
        const DNS_NS: [u8; 16] = [
            0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4,
            0x30, 0xc8,
        ];
        const EXPECTED: [u8; 16] = [
            0x6f, 0xa4, 0x59, 0xea, 0xee, 0x8a, 0x3c, 0xa4, 0x89, 0x4e, 0xdb, 0x77, 0xe1, 0x60,
            0x35, 0x5e,
        ];

        let ns = UUID { bytes: DNS_NS };
        let uuid = UUID::new_v3(&ns, "python.org");
        assert_eq!(uuid.bytes, EXPECTED);
    }
}
