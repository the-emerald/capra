use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// Measured in litres
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GasConsumption(pub u32);

impl Add for GasConsumption {
    type Output = GasConsumption;

    fn add(self, rhs: Self) -> Self::Output {
        GasConsumption(self.0 + rhs.0)
    }
}

impl AddAssign for GasConsumption {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for GasConsumption {
    type Output = GasConsumption;

    fn sub(self, rhs: Self) -> Self::Output {
        GasConsumption(self.0 - rhs.0)
    }
}

impl SubAssign for GasConsumption {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for GasConsumption {
    type Output = GasConsumption;

    fn mul(self, rhs: Self) -> Self::Output {
        GasConsumption(self.0 * rhs.0)
    }
}

impl MulAssign for GasConsumption {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for GasConsumption {
    type Output = GasConsumption;

    fn div(self, rhs: Self) -> Self::Output {
        GasConsumption(self.0 / rhs.0)
    }
}

impl DivAssign for GasConsumption {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}
