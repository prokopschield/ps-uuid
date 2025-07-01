use crate::{NodeId, UUID};

impl UUID {
    #[must_use]
    pub const fn get_node_id(&self) -> Option<NodeId> {
        match self.get_version() {
            Some(1 | 2 | 6) => {
                let [_, _, _, _, _, _, _, _, _, _, b1, b2, b3, b4, b5, b6] = self.bytes;

                let node_id = NodeId {
                    bytes: [b1, b2, b3, b4, b5, b6],
                };

                Some(node_id)
            }
            _ => None,
        }
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper that fabricates a UUID with a specific version *and* node ID.
    ///
    /// - `version` is written into the high nibble of byte 6  
    ///   (see RFC 4122 §4.1.3).
    /// - The RFC 4122 “variant 1” bits (`10xx_xxxx`) are written
    ///   into byte 8, but the variant is irrelevant for `node_id()`.
    fn make_uuid(version: u8, node: [u8; 6]) -> UUID {
        let mut bytes = [0u8; 16];

        // Any arbitrary values for the time / clock sequence fields:
        for (i, item) in bytes.iter_mut().enumerate() {
            *item = i.try_into().unwrap();
        }

        // Embed the requested version.
        bytes[6] = (version << 4) | (bytes[6] & 0x0F);

        // Embed RFC-4122 variant 1 (“10xx_xxxx”).
        bytes[8] = 0b1000_0000;

        // Copy the node identifier into the last six bytes.
        bytes[10..16].copy_from_slice(&node);

        UUID { bytes }
    }

    #[test]
    fn node_id_returns_some_for_versions_1_2_and_6() {
        let node = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];

        for &v in &[1u8, 2, 6] {
            let uuid = make_uuid(v, node);
            assert_eq!(
                uuid.get_node_id(),
                Some(NodeId { bytes: node }),
                "unexpected result for version {v}"
            );
        }
    }

    #[test]
    fn node_id_returns_none_for_all_other_versions() {
        let unwanted = [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];

        // All versions that must *not* yield a node ID.
        for &v in &[0u8, 3, 4, 5, 7, 8, 9, 0xF] {
            let uuid = make_uuid(v, unwanted);
            assert_eq!(
                uuid.get_node_id(),
                None,
                "version {v} incorrectly returned Some(..)"
            );
        }
    }

    // -----------------------------------------------------------------------
    // Compile-time proof: `UUID::node_id` is const-compatible
    // -----------------------------------------------------------------------

    // Build a UUID and compute its node ID entirely at compile time.
    const SAMPLE_UUID: UUID = {
        let mut bytes = [0u8; 16];

        bytes[6] = 0x10; // version 1
        bytes[8] = 0x80; // variant 1

        // node_id
        bytes[10] = 1;
        bytes[11] = 2;
        bytes[12] = 3;
        bytes[13] = 4;
        bytes[14] = 5;
        bytes[15] = 6;

        UUID { bytes }
    };

    const SAMPLE_NODE_ID: Option<NodeId> = SAMPLE_UUID.get_node_id();

    #[test]
    fn const_evaluation_matches_runtime_result() {
        assert_eq!(
            SAMPLE_NODE_ID,
            Some(NodeId {
                bytes: [1, 2, 3, 4, 5, 6]
            })
        );
    }
}
