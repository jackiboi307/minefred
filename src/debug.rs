use std::time::Instant;

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
        println!("timer: {} ms: {}", elapsed, self.label);
        elapsed
    }
}
