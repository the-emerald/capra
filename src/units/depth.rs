use crate::units::pressure::Pressure;
use crate::units::water_density::WaterDensity;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Depth(pub u32);

impl Depth {
    pub fn pressure(&self, density: WaterDensity) -> Pressure {
        Pressure((self.0 as f64 / density.0) + 1.0)
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
