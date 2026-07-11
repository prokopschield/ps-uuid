use crate::{NodeId, State};

impl State {
    /// Sets the node identifier embedded in generated UUIDs.
    ///
    /// This is the supported way to reseed the shared
    /// [`STATE`](crate::STATE) after a `fork()`; the timestamp and
    /// clock-sequence invariants are unaffected.
    pub const fn set_node_id(&mut self, node_id: NodeId) {
        self.node_id = node_id;
    }
}
