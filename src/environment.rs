use crate::units::altitude::Altitude;
use crate::units::water_density::WaterDensity;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Environment {
    water_density: WaterDensity,
    altitude: Altitude,
}

impl Environment {
    pub fn new(water_density: WaterDensity, altitude: Altitude) -> Self {
        Self {
            water_density,
            altitude,
        }
    }

    pub fn water_density(&self) -> WaterDensity {
        self.water_density
    }

    pub fn altitude(&self) -> Altitude {
        self.altitude
    }
}
