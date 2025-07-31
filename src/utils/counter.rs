pub struct Counter {
    i: u8,
    max: u8,
}

impl Counter {
    pub fn new(max: u8) -> Self {
        Self{i: max, max}
    }

    pub fn count(&mut self) -> bool {
        if self.i == self.max {
            self.i = 0;
            true
        } else {
            self.i += 1;
            false
        }
    }
}
