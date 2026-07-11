use std::time::{Duration, SystemTime};

use crate::State;

const FIDELITY: Duration = Duration::from_nanos(256);

impl State {
    /// This method returns the next `UUIDv7`'s timestamp.
    ///
    /// 1. Increments this [`State`]'s timestamp by 256 ns,
    /// 2. compares this with the timestamp provided, keeping the provided
    ///    timestamp only if it is greater and representable as a 60-bit
    ///    RFC 4122 tick count,
    /// 3. replaces this [`State`]'s timestamp with the value,
    /// 4. returns the value.
    ///
    /// # Usage
    ///
    /// ```
    /// use ps_uuid::STATE;
    /// use std::time::SystemTime;
    ///
    /// let next_ts = STATE.lock().next_v7(SystemTime::now());
    /// ```
    pub fn next_v7(&mut self, timestamp: SystemTime) -> SystemTime {
        // `stalled` is intentionally left alone: a stale count can only
        // shrink the next tick's clock-sequence budget, never enlarge it.
        let candidate = self.last_ts + FIDELITY;

        let timestamp = if timestamp > candidate && Self::is_adoptable(timestamp) {
            timestamp
        } else {
            candidate
        };

        self.last_ts = timestamp;

        timestamp
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use crate::{NodeId, State, STATE};

    #[test]
    fn always_increments() {
        let mut guard = STATE.lock_arc();

        let mut previous = guard.next_v7(SystemTime::now());

        for _ in 0..99999 {
            let next = guard.next_v7(SystemTime::now());

            assert!(next > previous, "Next timestamp must be greater.");

            previous = next;
        }

        drop(guard);
    }

    /// A reading beyond the representable 60-bit tick range self-advances by
    /// 256 ns instead of being adopted, preserving strict monotonicity.
    #[test]
    fn far_future_reading_self_advances() {
        let mut state = State {
            last_ts: UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: 0,
            stalled: 0,
            seq_v2: 0,
        };

        let bogus = UNIX_EPOCH + Duration::from_secs(200_000_000_000);

        let first = state.next_v7(bogus);
        let second = state.next_v7(bogus);

        assert_eq!(first, UNIX_EPOCH + Duration::from_nanos(256));
        assert_eq!(second, UNIX_EPOCH + Duration::from_nanos(512));

        let sane = UNIX_EPOCH + Duration::from_secs(1_000_000_000);

        let adopted = state.next_v7(sane);

        assert_eq!(adopted, sane);
    }
}
