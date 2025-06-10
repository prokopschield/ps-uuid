use crate::{NodeId, NODE_ID_BYTES};

impl From<&[u8; NODE_ID_BYTES]> for NodeId {
    fn from(bytes: &[u8; NODE_ID_BYTES]) -> Self {
        Self { bytes: *bytes }
    }
}

#[cfg(test)]
mod tests {
    use crate::NodeId;

    #[test]
    fn identity() {
        let bytes = &[1, 2, 3, 4, 5, 6];
        let node_id = NodeId::from(bytes);

        assert_eq!(*bytes, node_id.bytes, "Arrays should be identical.");
    }
}
