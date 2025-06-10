use std::ops::Deref;

use crate::{NodeId, NODE_ID_BYTES};

impl Deref for NodeId {
    type Target = [u8; NODE_ID_BYTES];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::NodeId;

    #[test]
    fn deref() {
        let id = NodeId::from([1, 2, 3, 4, 5, 6]);

        assert_eq!(id[..], [1, 2, 3, 4, 5, 6]);
    }
}
