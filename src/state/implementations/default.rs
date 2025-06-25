use chrono::DateTime;
use rand::random;

use crate::{NodeId, State};

impl Default for State {
    fn default() -> Self {
        Self {
            last_ts: DateTime::UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: random(),
        }
    }
}
