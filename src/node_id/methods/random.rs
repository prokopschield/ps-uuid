use rand::fill;

use crate::NodeId;

impl NodeId {
    /// Generates a random [`NodeId`] with the multicast bit set.
    #[must_use]
    pub fn random() -> Self {
        let mut bytes = [0u8; 6];

        fill(&mut bytes);

        // Set the multicast bit (least significant bit of the first byte)
        bytes[0] |= 0x01;

        Self { bytes }
    }
}

#[cfg(test)]
mod tests {
    use crate::NodeId;

    #[test]
    fn unique() {
        let list: Vec<NodeId> = (0..10).map(|_| NodeId::random()).collect();

        for (index, item) in list.iter().enumerate() {
            let loop_index = list.iter().position(|loop_item| loop_item == item);

            assert_eq!(Some(index), loop_index, "Each node_id should be unique!");
        }
    }

    #[test]
    fn multicast_bit_is_set() {
        for _ in 0..100 {
            let node = NodeId::random();
            assert_eq!(
                node.bytes[0] & 0x01,
                0x01,
                "Multicast bit (LSB of first byte) must be set"
            );
        }
    }
}
