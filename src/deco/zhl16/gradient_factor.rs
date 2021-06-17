#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
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

    pub fn fr_low(&self) -> f64 {
        self.low as f64 / 100.0
    }

    pub fn high(&self) -> u8 {
        self.high
    }

    pub fn fr_high(&self) -> f64 {
        self.high as f64 / 100.0
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
