use crate::common::depth::Depth;
use crate::common::water_density::WaterDensity;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Copy, Clone, Default, Debug, PartialOrd, PartialEq)]
pub struct Pressure(pub f64);

impl Pressure {
    pub fn equivalent_depth(&self, density: &WaterDensity) -> Depth {
        Depth(((self.0 - 1.0) * density.meters_per_bar()) as u32)
    }
}

impl Add for Pressure {
    type Output = Pressure;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output(self.0 + rhs.0)
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
        Self::Output(self.0 - rhs.0)
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
        Self::Output(self.0 * rhs.0)
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
        Self::Output(self.0 / rhs.0)
    }
}

impl DivAssign for Pressure {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}