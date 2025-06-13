mod methods;

use std::time::Duration;

const GREGORIAN_OFFSET_SECONDS: u64 = 0x0002_d853_9c80; // Offset from 1582-10-15 to 1970-01-01 in seconds
const GREGORIAN_OFFSET: Duration = Duration::new(GREGORIAN_OFFSET_SECONDS, 0);

pub struct Gregorian;
