use crate::units::depth::Depth;
use crate::units::pressure::Pressure;
use crate::environment::Environment;

// TODO: Gas error types
#[derive(Debug)]
pub enum GasError {
    FractionError,
}

pub struct Gas {
    o2: u32,
    he: u32,
    n2: u32,
}

impl Gas {
    pub fn new(o2: u32, he: u32, n2: u32) -> Result<Self, GasError> {
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

    pub fn o2(&self) -> u32 {
        self.o2
    }

    pub fn n2(&self) -> u32 {
        self.n2
    }

    pub fn he(&self) -> u32 {
        self.he
    }

    pub fn equivalent_narcotic_depth(&self, depth: Depth) -> Depth {
        Depth(((depth.0 + 10) as f64 * (1.0 - self.fr_he()) - 10.0) as u32)
    }

    pub fn ppo2_in_range(&self, depth: Depth, min: Pressure, max: Pressure, environment: Environment) -> bool {
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
}
