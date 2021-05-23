pub struct GradientFactor {
    low: u32,
    high: u32,
}

impl Default for GradientFactor {
    fn default() -> Self {
        Self {
            low: 100,
            high: 100
        }
    }
}