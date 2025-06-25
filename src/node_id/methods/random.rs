use rand::fill;

use crate::NodeId;

impl NodeId {
    /// Generates a random [`NodeId`].
    #[must_use]
    pub fn random() -> Self {
        let mut bytes = [0u8; 6];

        fill(&mut bytes);

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
}
