use std::time::Instant;

const PRINT_DEBUG: bool = true;
const PRINT_TIMER: bool = false;
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

    pub fn done_cond(&self, cond: bool) -> u128 {
        let elapsed = self.elapsed();
        if cond && !(DO_NOT_PRINT_0_MS && elapsed == 0) {
            println!("timer: {} ms: {}", elapsed, self.label);
        }
        elapsed
    }

    pub fn done(&self) -> u128 {
        self.done_cond(PRINT_DEBUG && PRINT_TIMER)
    }
}
