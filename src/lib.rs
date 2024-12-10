/// microns is a simple, dependency-free, library to handling floats as fixed precision ints.
/// microns gets its name from converting millimeter formatted f32 to an int
/// with 10e-6 precision, but can be used in any case where i32::MIN < float < i32::MAX.
/// This is useful for working with CNC machines, 3D printers, or any situation where
/// micron precision is adequate and representations are traditionally formatted as floats.
use std::ops::{Add, Div, Mul, Sub};

pub fn works(val: f32) -> bool {
    if val.is_nan() {
        return false;
    }
    val > f32::from(Microns::MIN) && val < f32::from(Microns::MAX)
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Microns is a simple struct that holds a i32 value,
/// meant to be converted from a float for simplified math.
pub struct Microns(pub i32);

impl Microns {
    pub const ZERO: Microns = Microns(0);
    pub const MIN: Microns = Microns(i32::MIN);
    pub const MAX: Microns = Microns(i32::MAX);

    pub fn abs(&self) -> Self {
        Microns(self.0.abs())
    }
}
impl From<f32> for Microns {
    fn from(other: f32) -> Self {
        assert!(works(other), "Value out of range");
        Microns((other * 1000.0).trunc() as i32)
    }
}

impl From<Microns> for f32 {
    fn from(other: Microns) -> Self {
        other.0 as f32 / 1000.0
    }
}

impl Add for Microns {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Microns(self.0 + rhs.0)
    }
}

impl Add<f32> for Microns {
    type Output = Self;
    fn add(self, rhs: f32) -> Self {
        self + Microns::from(rhs)
    }
}

impl Sub for Microns {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Microns(self.0 - rhs.0)
    }
}

impl Sub<f32> for Microns {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self {
        self - Microns::from(rhs)
    }
}

impl Mul<f32> for Microns {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Microns::from(f32::from(self) * rhs)
    }
}

impl Div<f32> for Microns {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Microns::from(f32::from(self) / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Microns(1);
        let b = Microns(2);
        let c = a + b;
        assert_eq!(c, Microns(3));
    }

    #[test]
    fn test_sub() {
        let a = Microns(1);
        let b = Microns(2);
        let c = a - b;
        assert_eq!(c, Microns(-1));
    }

    #[test]
    fn test_add_f32() {
        let a = Microns(1);
        let b = 0.002;
        let c = a + b;
        assert_eq!(c, Microns(3));
    }

    #[test]
    fn test_sub_f32() {
        let a = Microns(1);
        let b = 0.002;
        let c = a - b;
        assert_eq!(c, Microns(-1));
    }

    #[test]
    fn test_from_f32() {
        let a = 0.001;
        let b = Microns::from(a);
        assert_eq!(b, Microns(1));
    }

    #[test]
    fn test_into_f32() {
        let a = Microns(1);
        let b: f32 = a.into();
        assert_eq!(b, 0.001);
    }

    #[test]
    fn test_mul_f32() {
        let a = Microns(1);
        let b = 2.0;
        let c = a * b;
        assert_eq!(c, Microns(2));
    }

    #[test]
    fn test_div_f32() {
        let a = Microns(1);
        let b = 2.0;
        let c = a / b;
        assert_eq!(c, Microns(0));
        let a = Microns(10);
        let b = 2.0;
        let c = a / b;
        assert_eq!(c, Microns(5));
    }

    #[test]
    fn test_abs() {
        assert_eq!(Microns(-1).abs(), Microns(1));
        assert_eq!(Microns(-1111).abs(), Microns(1111));
        assert_eq!(Microns(0).abs(), Microns(0));
        assert_eq!(Microns(666).abs(), Microns(666));
    }
}
