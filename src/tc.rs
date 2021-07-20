use num_complex::Complex64;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TC {
    value: Complex64,
    derivative: Complex64,
}

impl TC {
    pub fn new(value: Complex64, derivative: Complex64) -> Self {
        Self { value, derivative }
    }
}

impl From<f64> for TC {
    fn from(x: f64) -> Self {
        Self {
            value: Complex64::new(x, 0.0),
            derivative: Complex64::new(0.0, 0.0),
        }
    }
}

impl From<Complex64> for TC {
    fn from(z: Complex64) -> Self {
        Self {
            value: z,
            derivative: Complex64::new(0.0, 0.0),
        }
    }
}

impl Add for TC {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            derivative: self.derivative + rhs.derivative,
        }
    }
}

impl AddAssign for TC {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        self.derivative += rhs.derivative;
    }
}

impl Sub for TC {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
            derivative: self.derivative - rhs.derivative,
        }
    }
}

impl SubAssign for TC {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
        self.derivative -= rhs.derivative;
    }
}

impl Mul for TC {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
            derivative: self.value * rhs.derivative + self.derivative * rhs.value,
        }
    }
}

impl MulAssign for TC {
    fn mul_assign(&mut self, rhs: Self) {
        self.derivative = self.value * rhs.derivative + self.derivative * rhs.value;
        self.value = self.value * rhs.value;
    }
}

// impl DivAssign for TC {
//     fn div_assign(&mut self, rhs: Self) {
//         let nr = 1.0/rhs.value.norm_sqr()
//     }
// }
