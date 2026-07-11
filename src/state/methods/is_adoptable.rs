use std::time::SystemTime;

use crate::{State, UUID};

impl State {
    /// Returns whether a clock reading may be adopted as the shared last
    /// timestamp: it must be representable as a 60-bit RFC 4122 tick count
    /// (on or after 1582-10-15 and before roughly the year 5236).
    ///
    /// Rejecting unrepresentable readings ensures adoption alone never moves
    /// [`State::last_ts`] out of the range the time-based constructors
    /// accept, so one bogus reading from a broken clock cannot permanently
    /// poison the shared state.
    pub(crate) fn is_adoptable(timestamp: SystemTime) -> bool {
        UUID::system_time_to_ticks(timestamp).is_ok()
    }
}
