mod implementations;
mod methods;

use std::{sync::Arc, time::SystemTime};

use parking_lot::Mutex;

use crate::NodeId;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    pub last_ts: SystemTime,
    pub node_id: NodeId,
    pub seq: u16,
}

pub static STATE: std::sync::LazyLock<Arc<Mutex<State>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(State::default())));
