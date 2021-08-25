/// Average density of salt water (measured in kg m^-3).
pub const SALTWATER: WaterDensity = WaterDensity(1023.6);

/// Density of fresh water (measured in kg m^-3).
pub const FRESHWATER: WaterDensity = WaterDensity(997.0);

#[derive(Copy, Clone, Default, Debug, PartialOrd, PartialEq)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WaterDensity(pub f64);

impl WaterDensity {
    pub fn meters_per_bar(&self) -> f64 {
        10000.0 / self.0
    }
}
