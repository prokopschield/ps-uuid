use crate::{NodeId, NODE_ID_BYTES};

impl NodeId {
    /// Returns a reference to the inner byte array.
    #[must_use]
    pub fn as_bytes_mut(&mut self) -> &mut [u8; NODE_ID_BYTES] {
        &mut self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::NodeId;

    #[test]
    fn as_bytes_mut() {
        let mut node_id = NodeId::from([1, 2, 3, 4, 5, 6]);

        node_id.as_bytes_mut()[5] = 22;

        assert_eq!(node_id.bytes, [1, 2, 3, 4, 5, 22]);
    }
}
