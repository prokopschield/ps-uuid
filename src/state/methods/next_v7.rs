use std::time::{Duration, SystemTime};

use crate::State;

const FIDELITY: Duration = Duration::from_nanos(256);

impl State {
    /// This method returns the next `UUIDv7`'s timestamp.
    ///
    /// 1. Increments this [`State`]'s timestamp by 256 ns,
    /// 2. compares this with the timestamp provided, keeping the greater of the two,
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
        let timestamp = timestamp.max(self.last_ts + FIDELITY);

        self.last_ts = timestamp;

        timestamp
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use crate::STATE;

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
}
