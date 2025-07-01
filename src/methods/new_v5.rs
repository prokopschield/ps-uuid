use crate::{Sha1, UUID};

impl UUID {
    /// Builds an RFC-4122 Version-5 UUID from `namespace || name`.
    ///
    /// Steps
    /// 1. Hash `namespace.bytes || name` with SHA-1.
    /// 2. Pass the first 16 bytes of the digest to `from_parts_v5`.
    #[must_use]
    pub fn new_v5<N>(namespace: &Self, name: N) -> Self
    where
        N: AsRef<[u8]>,
    {
        let mut hasher = Sha1::new();

        hasher.update(namespace.as_bytes());
        hasher.update(name.as_ref());

        let digest = hasher.finalize();

        Self::from_parts_v5(&digest[..16])
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test-suite
// ────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sha1;

    const fn is_rfc4122_variant(b: u8) -> bool {
        (b & 0b1100_0000) == 0b1000_0000
    }

    #[test]
    fn version_and_variant_are_correct() {
        let ns = UUID { bytes: [0u8; 16] };
        let uuid = UUID::new_v5(&ns, "abc");
        assert_eq!(uuid.get_version(), Some(5));
        assert!(is_rfc4122_variant(uuid.bytes[8]));
    }

    #[test]
    fn identical_inputs_yield_identical_uuids() {
        let ns = UUID {
            bytes: [0x55u8; 16],
        };
        let name = b"the-same-name";
        assert_eq!(
            UUID::new_v5(&ns, name).bytes,
            UUID::new_v5(&ns, name).bytes,
            "Deterministic output expected"
        );
    }

    #[test]
    fn matches_manual_digest() {
        let ns = UUID { bytes: [7u8; 16] };
        let name = b"xyz";

        // via API
        let via_api = UUID::new_v5(&ns, name);

        // Manual SHA-1 → from_parts_v5
        let mut h = Sha1::new();
        h.update(ns.as_bytes());
        h.update(name);
        let digest = h.finalize();
        let mut dig = [0u8; 20];
        dig.copy_from_slice(&digest);
        let via_parts = UUID::from_parts_v5(dig);

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
            0x88, 0x63, 0x13, 0xe1, 0x3b, 0x8a, 0x53, 0x72, 0x9b, 0x90, 0x0c, 0x9a, 0xee, 0x19,
            0x9e, 0x5d,
        ];

        let ns = UUID { bytes: DNS_NS };
        let uuid = UUID::new_v5(&ns, "python.org");
        assert_eq!(uuid.bytes, EXPECTED);
    }
}
