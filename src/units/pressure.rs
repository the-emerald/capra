use crate::environment::Environment;
use crate::units::depth::Depth;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A default, placeholder minimum ppO2.
pub const PPO2_MINIMUM: Pressure = Pressure(0.18);

/// A default, placeholder maximum ppO2 for use during bottom segments.
pub const PPO2_MAXIMUM_DIVE: Pressure = Pressure(1.4);

/// A default, placeholder maximum ppO2 for use during decompression stops.
pub const PPO2_MAXIMUM_DECO: Pressure = Pressure(1.6);

/// A "fudge factor" added to ppO2 limits to account for water pressure differences.
pub const PPO2_FUDGE_FACTOR: Pressure = Pressure(0.1);

pub const WATER_VAPOUR_PRESSURE: Pressure = Pressure(0.06257);

#[derive(Copy, Clone, Default, Debug, PartialOrd, PartialEq)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pressure(pub f64);

impl Pressure {
    pub fn equivalent_depth(&self, environment: Environment) -> Depth {
        Depth(((self.0 - 1.0) * environment.water_density().meters_per_bar()) as u32)
    }
}

impl Add for Pressure {
    type Output = Pressure;

    fn add(self, rhs: Self) -> Self::Output {
        Pressure(self.0 + rhs.0)
    }
}

impl AddAssign for Pressure {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Pressure {
    type Output = Pressure;

    fn sub(self, rhs: Self) -> Self::Output {
        Pressure(self.0 - rhs.0)
    }
}

impl SubAssign for Pressure {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Pressure {
    type Output = Pressure;

    fn mul(self, rhs: Self) -> Self::Output {
        Pressure(self.0 * rhs.0)
    }
}

impl MulAssign for Pressure {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for Pressure {
    type Output = Pressure;

    fn div(self, rhs: Self) -> Self::Output {
        Pressure(self.0 / rhs.0)
    }
}

impl DivAssign for Pressure {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}
