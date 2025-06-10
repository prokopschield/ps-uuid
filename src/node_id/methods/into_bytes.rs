use crate::{NodeId, NODE_ID_BYTES};

impl NodeId {
    /// Returns the inner byte array.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; NODE_ID_BYTES] {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::NodeId;

    #[test]
    fn into_bytes() {
        let node_id = NodeId::from([1, 2, 3, 4, 5, 6]);

        assert_eq!(node_id.into_bytes(), [1, 2, 3, 4, 5, 6]);
    }
}
