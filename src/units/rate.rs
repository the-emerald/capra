use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rate(pub i32);

impl Add for Rate {
    type Output = Rate;

    fn add(self, rhs: Self) -> Self::Output {
        Rate(self.0 + rhs.0)
    }
}

impl AddAssign for Rate {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Rate {
    type Output = Rate;

    fn sub(self, rhs: Self) -> Self::Output {
        Rate(self.0 - rhs.0)
    }
}

impl SubAssign for Rate {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Rate {
    type Output = Rate;

    fn mul(self, rhs: Self) -> Self::Output {
        Rate(self.0 * rhs.0)
    }
}

impl MulAssign for Rate {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for Rate {
    type Output = Rate;

    fn div(self, rhs: Self) -> Self::Output {
        Rate(self.0 / rhs.0)
    }
}

impl DivAssign for Rate {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}
