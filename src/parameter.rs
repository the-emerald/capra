use crate::environment::Environment;
use crate::units::consumption_rate::GasConsumptionRate;
use crate::units::rate::Rate;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameters {
    ascent_rate: Rate,
    descent_rate: Rate,
    environment: Environment,
    sac_bottom: GasConsumptionRate,
    sac_deco: GasConsumptionRate,
}

impl Parameters {
    pub fn new(
        ascent_rate: Rate,
        descent_rate: Rate,
        environment: Environment,
        sac_bottom: GasConsumptionRate,
        sac_deco: GasConsumptionRate,
    ) -> Self {
        Parameters {
            ascent_rate,
            descent_rate,
            environment,
            sac_bottom,
            sac_deco,
        }
    }
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
    pub fn sac_bottom(&self) -> GasConsumptionRate {
        self.sac_bottom
    }
    pub fn sac_deco(&self) -> GasConsumptionRate {
        self.sac_deco
    }
}
