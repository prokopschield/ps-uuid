use std::time::{Duration, SystemTime};

use crate::State;

const FIDELITY: Duration = Duration::from_nanos(100);

impl State {
    /// This method returns the next [`UUID`](crate::UUID)'s timestamp and clock sequence.
    ///
    /// 1. Compares the provided timestamp with this [`State`]'s last timestamp,
    /// 2. increments the clock sequence modulo 2¹⁴ unless the provided
    ///    timestamp is more than 100 ns later, so both same-tick calls and a
    ///    backward-moving clock advance the sequence,
    /// 3. stores the provided timestamp as the last timestamp,
    /// 4. returns the provided timestamp and the clock sequence.
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
