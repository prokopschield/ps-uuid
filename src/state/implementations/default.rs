use rand::random;

use crate::{NodeId, State};

impl Default for State {
    fn default() -> Self {
        Self {
            last_ts: 0,
            node_id: NodeId::random(),
            seq: random(),
        }
    }
}
