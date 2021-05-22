pub struct GradientFactor {
    low: u8,
    high: u8,
}

impl GradientFactor {
    pub fn new(low: u8, high: u8) -> Self {
        Self { low, high }
    }

    pub fn low(&self) -> u8 {
        self.low
    }

    pub fn high(&self) -> u8 {
        self.high
    }
}

impl Default for GradientFactor {
    fn default() -> Self {
        Self {
            low: 100,
            high: 100,
        }
    }
}
