use crate::{NodeId, NODE_ID_BYTES};

impl AsMut<[u8; NODE_ID_BYTES]> for NodeId {
    fn as_mut(&mut self) -> &mut [u8; NODE_ID_BYTES] {
        &mut self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::NodeId;

    #[test]
    fn identity() {
        let mut node_id = NodeId::from([1, 2, 3, 4, 5, 6]);

        node_id.as_mut()[3] = 42;

        assert_eq!(
            node_id.bytes,
            [1, 2, 3, 42, 5, 6],
            "NodeId should be modified."
        );
    }
}
