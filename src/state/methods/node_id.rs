use crate::{NodeId, State};

impl State {
    /// Returns the node identifier embedded in generated UUIDs.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }
}
