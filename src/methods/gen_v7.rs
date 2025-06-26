use std::time::{SystemTime, UNIX_EPOCH};

use rand::random;

use crate::{UuidConstructionError, STATE, UUID};

impl UUID {
    /// Generate an RFC-4122 **Version 7** (Unix-epoch, time-ordered) UUID.
    ///
    /// Steps  
    /// 1. `STATE.next_v7` returns a strictly monotonous `SystemTime`.  
    /// 2. That time is converted to a `Duration` since the Unix epoch.  
    /// 3. Range checks ensure the 48-bit millisecond field is valid
    ///    (epoch … ≈ 10889-08-02 05:31:50.655 UTC).  
    /// 4. The remaining **64 random bits (8 bytes)** are filled with CSPRNG
    ///    data.  
    /// 5. `UUID::new_v7` assembles the final UUID and patches
    ///    version & variant bits.
    ///
    /// # Errors
    /// - `TimestampBeforeEpoch`  if the system clock is before 1970-01-01.
    /// - `TimestampOverflow`     if the millisecond counter ≥ 2⁴⁸.
    pub fn gen_v7() -> Result<Self, UuidConstructionError> {
        // 1 — obtain monotonic timestamp
        let timestamp = {
            let mut guard = STATE.lock();
            let ts = guard.next_v7(SystemTime::now());
            drop(guard);
            ts
        };

        // 2 — convert to Duration and validate range
        let duration = timestamp
            .duration_since(UNIX_EPOCH)
            .map_err(|_| UuidConstructionError::TimestampBeforeEpoch)?;

        #[allow(clippy::items_after_statements)]
        const MAX_MILLIS: u128 = 1u128 << 48; // 2⁴⁸ ms
        if duration.as_millis() >= MAX_MILLIS {
            return Err(UuidConstructionError::TimestampOverflow);
        }

        // 3 — 64 bits (8 bytes) of randomness
        let random_bytes: [u8; 8] = random();

        // 4 — assemble
        Ok(Self::new_v7(duration, random_bytes))
    }
}

#[allow(clippy::expect_used, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashSet,
        sync::{Arc, Mutex},
        thread,
    };

    const fn version(b: &[u8; 16]) -> u8 {
        b[6] >> 4
    }
    const fn variant(b: &[u8; 16]) -> u8 {
        b[8] >> 6
    }

    #[test]
    fn gen_v7_produces_valid_uuid() {
        let uuid = UUID::gen_v7().expect("generation must succeed");
        let bytes = uuid.as_bytes();
        assert_eq!(version(bytes), 0b0111);
        assert_eq!(variant(bytes), 0b10);
    }

    #[test]
    fn gen_v7_uniqueness_single_thread() {
        const N: usize = 10_000;
        let mut seen = HashSet::with_capacity(N);
        for _ in 0..N {
            let s = UUID::gen_v7().unwrap().to_string();
            assert!(seen.insert(s), "duplicate UUID generated");
        }
    }

    #[test]
    fn gen_v7_thread_safety_and_uniqueness() {
        const THREADS: usize = 8;
        const PER_THREAD: usize = 2_000;

        let global: Arc<Mutex<HashSet<UUID>>> =
            Arc::new(Mutex::new(HashSet::with_capacity(THREADS * PER_THREAD)));

        let mut handles = Vec::with_capacity(THREADS);
        for _ in 0..THREADS {
            let global = Arc::clone(&global);
            handles.push(thread::spawn(move || {
                for _ in 0..PER_THREAD {
                    let id = UUID::gen_v7().expect("generation must succeed");
                    let mut guard = global.lock().unwrap();
                    assert!(guard.insert(id), "duplicate across threads");
                    drop(guard);
                }
            }));
        }
        for h in handles {
            h.join().expect("thread panicked");
        }
    }
}
