use crate::{NodeId, NODE_ID_BYTES};

impl AsRef<[u8; NODE_ID_BYTES]> for NodeId {
    fn as_ref(&self) -> &[u8; NODE_ID_BYTES] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::NodeId;

    #[test]
    fn identity() {
        let bytes = [1, 2, 3, 4, 5, 6];
        let node_id = NodeId::from(bytes);

        assert_eq!(
            bytes.as_ref(),
            &node_id.bytes,
            "Arrays should be identical."
        );
    }
}
