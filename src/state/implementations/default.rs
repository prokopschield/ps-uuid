use std::time::{Duration, UNIX_EPOCH};

use rand::random;

use crate::{gregorian::GREGORIAN_OFFSET, methods::FILETIME_EPOCH_OFFSET, NodeId, State};

/// The offset from 1601-01-01 (the `FILETIME` epoch, and the exact floor of
/// `SystemTime` on Windows) to 1970-01-01.
const FILETIME_OFFSET: Duration = Duration::from_secs(FILETIME_EPOCH_OFFSET / 10_000_000);

impl Default for State {
    fn default() -> Self {
        // Floor the state at the earliest instant the platform clock can
        // represent: the UUID epoch (1582-10-15) where representable, else
        // the FILETIME epoch (1601-01-01, the exact floor on Windows), else
        // the Unix epoch (targets whose clock starts at 1970-01-01, such as
        // wasm32-unknown-unknown). Every clock reading the platform can
        // produce compares greater than the floor, so a fresh state adopts
        // the first sane reading it sees.
        let floor = UNIX_EPOCH
            .checked_sub(GREGORIAN_OFFSET)
            .or_else(|| UNIX_EPOCH.checked_sub(FILETIME_OFFSET))
            .unwrap_or(UNIX_EPOCH);

        Self {
            last_ts: floor,
            node_id: NodeId::random(),
            seq: random(),
            stalled: 0,
            seq_v2: random(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use crate::{State, UUID};

    #[test]
    fn fresh_state_floor_is_tick_representable_and_pre_unix() {
        let state = State::default();

        assert!(
            state.last_ts <= UNIX_EPOCH,
            "the floor must not exceed the Unix epoch"
        );
        assert!(
            UUID::system_time_to_ticks(state.last_ts).is_ok(),
            "the floor must be representable as a 60-bit tick count"
        );
    }
}
