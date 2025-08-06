use std::time::Instant;

const PRINT_DEBUG: bool = true;
const PRINT_TIMER: bool = false && PRINT_DEBUG;
const DO_NOT_PRINT_0_MS: bool = true;
pub const PRINT_LAG: bool = false;

pub struct Timer {
    label: &'static str,
    start: Instant,
}

impl Timer {
    pub fn new(label: &'static str) -> Self {
        Self{
            label,
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> u128 {
        self.start.elapsed().as_millis()
    }

    pub fn done(&self) -> u128 {
        let elapsed = self.elapsed();
        if PRINT_TIMER && !(DO_NOT_PRINT_0_MS && elapsed == 0) {
            println!("timer: {} ms: {}", elapsed, self.label);
        }
        elapsed
    }
}
