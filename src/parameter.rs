use crate::environment::Environment;
use crate::units::air_consumption::AirConsumption;
use crate::units::rate::Rate;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameters {
    ascent_rate: Rate,
    descent_rate: Rate,
    environment: Environment,
    sac_bottom: AirConsumption,
    sac_deco: AirConsumption,
}

impl Parameters {
    pub fn ascent_rate(&self) -> Rate {
        self.ascent_rate
    }
    pub fn descent_rate(&self) -> Rate {
        self.descent_rate
    }
    pub fn environment(&self) -> Environment {
        self.environment
    }
    pub fn sac_bottom(&self) -> AirConsumption {
        self.sac_bottom
    }
    pub fn sac_deco(&self) -> AirConsumption {
        self.sac_deco
    }
}
