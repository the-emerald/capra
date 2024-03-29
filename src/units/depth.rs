use crate::environment::Environment;
use crate::units::pressure::{Pressure, WATER_VAPOUR_PRESSURE};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Depth(pub u32);

impl Depth {
    pub fn pressure(&self, environment: Environment) -> Pressure {
        Pressure(self.0 as f64 / environment.water_density().meters_per_bar())
            + environment.altitude().atmospheric_pressure()
    }

    pub fn compensated_pressure(&self, environment: Environment) -> Pressure {
        self.pressure(environment) - WATER_VAPOUR_PRESSURE
    }

    pub fn delta(&self, rhs: Depth) -> Depth {
        *self.max(&rhs) - *self.min(&rhs)
    }
}

impl Add for Depth {
    type Output = Depth;

    fn add(self, rhs: Self) -> Self::Output {
        Depth(self.0 + rhs.0)
    }
}

impl AddAssign for Depth {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Depth {
    type Output = Depth;

    fn sub(self, rhs: Self) -> Self::Output {
        Depth(self.0 - rhs.0)
    }
}

impl SubAssign for Depth {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Depth {
    type Output = Depth;

    fn mul(self, rhs: Self) -> Self::Output {
        Depth(self.0 * rhs.0)
    }
}

impl MulAssign for Depth {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for Depth {
    type Output = Depth;

    fn div(self, rhs: Self) -> Self::Output {
        Depth(self.0 / rhs.0)
    }
}

impl DivAssign for Depth {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}
