use std::ops::DerefMut;

use crate::NodeId;

impl DerefMut for NodeId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::NodeId;

    #[test]
    fn deref_mut() {
        let mut node_id = NodeId::from([1, 2, 3, 4, 5, 6]);

        node_id[2] = 16;

        assert_eq!(
            node_id.bytes,
            [1, 2, 16, 4, 5, 6],
            "NodeId should be modified."
        );
    }
}
