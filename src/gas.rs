use crate::environment::Environment;
use crate::units::depth::Depth;
use crate::units::pressure::Pressure;
use thiserror::Error;

#[derive(Copy, Clone, Debug, Error, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum GasError {
    #[error("gas fractions do not add up to 100")]
    FractionError,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gas {
    o2: u8,
    he: u8,
    n2: u8,
}

impl Gas {
    pub fn new(o2: u8, he: u8, n2: u8) -> Result<Self, GasError> {
        if o2 + he + n2 != 100 {
            Err(GasError::FractionError)
        } else {
            Ok(Self { o2, he, n2 })
        }
    }

    pub fn fr_n2(&self) -> f64 {
        self.n2 as f64 / 100.0
    }

    pub fn fr_o2(&self) -> f64 {
        self.o2 as f64 / 100.0
    }

    pub fn fr_he(&self) -> f64 {
        self.he as f64 / 100.0
    }

    pub fn o2(&self) -> u8 {
        self.o2
    }

    pub fn n2(&self) -> u8 {
        self.n2
    }

    pub fn he(&self) -> u8 {
        self.he
    }

    pub fn equivalent_narcotic_depth(&self, depth: Depth) -> Depth {
        Depth(((depth.0 + 10) as f64 * (1.0 - self.fr_he()) - 10.0) as u32)
    }

    pub fn ppo2_in_range(
        &self,
        depth: Depth,
        min: Pressure,
        max: Pressure,
        environment: Environment,
    ) -> bool {
        let ppo2 = self.pp_o2(depth, environment);
        ppo2 >= min && ppo2 <= max
    }

    pub fn pp_o2(&self, depth: Depth, environment: Environment) -> Pressure {
        Pressure(depth.pressure(environment).0 * self.fr_o2())
    }

    pub fn pp_he(&self, depth: Depth, environment: Environment) -> Pressure {
        Pressure(depth.pressure(environment).0 * self.fr_he())
    }

    pub fn pp_n2(&self, depth: Depth, environment: Environment) -> Pressure {
        Pressure(depth.pressure(environment).0 * self.fr_n2())
    }

    pub fn max_operating_depth(&self, max_pp_o2: Pressure, environment: Environment) -> Depth {
        Pressure(max_pp_o2.0 / self.fr_o2()).equivalent_depth(environment)
    }
}
