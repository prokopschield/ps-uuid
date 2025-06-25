use rand::random;

use crate::{Gregorian, NodeId, State};

impl Default for State {
    fn default() -> Self {
        Self {
            last_ts: Gregorian::epoch(),
            node_id: NodeId::random(),
            seq: random(),
        }
    }
}
