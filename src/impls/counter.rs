pub struct LoopCounter {
    count: f64,
    ubound: f64,
}

impl LoopCounter {
    pub fn new(ubound: f64) -> Self {
        LoopCounter { count: ubound - 0.5, ubound }
    }
}

impl Iterator for LoopCounter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 0.5;

        if self.count > self.ubound {
            self.count = 0.5;
        }

        Some(self.count)
    }
}