mod implementations;

use std::{sync::Arc, time::SystemTime};

use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::NodeId;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    pub last_ts: SystemTime,
    pub node_id: NodeId,
    pub seq: u16,
}

pub static STATE: Lazy<Arc<Mutex<State>>> = Lazy::new(|| Arc::new(Mutex::new(State::default())));
