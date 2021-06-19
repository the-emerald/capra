use crate::units::pressure::Pressure;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Altitude(pub u32);

impl Altitude {
    pub fn atmospheric_pressure(&self) -> Pressure {
        Pressure(101.325 * (-0.00012_f64 * self.0 as f64).exp() / 100.0)
    }
}

impl Add for Altitude {
    type Output = Altitude;

    fn add(self, rhs: Self) -> Self::Output {
        Altitude(self.0 + rhs.0)
    }
}

impl AddAssign for Altitude {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Altitude {
    type Output = Altitude;

    fn sub(self, rhs: Self) -> Self::Output {
        Altitude(self.0 - rhs.0)
    }
}

impl SubAssign for Altitude {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Altitude {
    type Output = Altitude;

    fn mul(self, rhs: Self) -> Self::Output {
        Altitude(self.0 * rhs.0)
    }
}

impl MulAssign for Altitude {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for Altitude {
    type Output = Altitude;

    fn div(self, rhs: Self) -> Self::Output {
        Altitude(self.0 / rhs.0)
    }
}

impl DivAssign for Altitude {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}
