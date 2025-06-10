use crate::{NodeId, NODE_ID_BYTES};

impl NodeId {
    #[must_use]
    pub const fn from_bytes(bytes: [u8; NODE_ID_BYTES]) -> Self {
        Self { bytes }
    }
}

#[cfg(test)]
mod tests {
    use crate::NodeId;

    #[test]
    fn identity() {
        let bytes = [1, 2, 3, 4, 5, 6];
        let node_id = NodeId::from_bytes(bytes);

        assert_eq!(bytes, node_id.bytes, "Arrays should be identical.");
    }
}
