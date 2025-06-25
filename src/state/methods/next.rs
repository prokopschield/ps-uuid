use std::time::{Duration, SystemTime};

use crate::State;

const FIDELITY: Duration = Duration::from_nanos(100);

impl State {
    /// This method returns the next [`UUID`]'s timestamp and clock sequence.
    ///
    /// 1. Increments this [`State`]'s timestamp by 100 ns,
    /// 2. compares this with the timestamp provided,
    /// 3. if the provided timestamp is greater or equal, the clock sequence is incremented,
    /// 4. the timestamp and clock sequence are returned.
    ///
    /// # Usage
    ///
    /// ```
    /// use ps_uuid::STATE;
    /// use std::time::SystemTime;
    ///
    /// let (timestamp, clock_seq) = STATE.lock().next(SystemTime::now());
    /// ```
    pub fn next(&mut self, timestamp: SystemTime) -> (SystemTime, u16) {
        if timestamp <= self.last_ts + FIDELITY {
            self.seq = (self.seq.wrapping_add(1)) & 0x3FFF;
        }

        self.last_ts = timestamp;

        (timestamp, self.seq)
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use crate::STATE;

    #[test]
    fn always_increments() {
        let mut guard = STATE.lock_arc();

        let mut previous = guard.next(SystemTime::now());

        for _ in 0..99999 {
            let next = guard.next(SystemTime::now());

            assert!(
                next > previous,
                "Next timestamp or sequence number must be greater."
            );

            previous = next;
        }

        drop(guard);
    }
}
