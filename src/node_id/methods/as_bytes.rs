use crate::{NodeId, NODE_ID_BYTES};

impl NodeId {
    /// Returns a reference to the inner byte array.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; NODE_ID_BYTES] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::NodeId;

    #[test]
    fn as_bytes() {
        let id = NodeId::from([1, 2, 3, 4, 5, 6]);

        assert_eq!(id.as_bytes(), &[1, 2, 3, 4, 5, 6]);
    }
}
