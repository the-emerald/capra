#[derive(Copy, Clone, Default, Debug, PartialOrd, PartialEq)]
pub struct WaterDensity(pub f64);

impl WaterDensity {
    pub fn meters_per_bar(&self) -> f64 {
        10000.0 / self.0
    }
}
