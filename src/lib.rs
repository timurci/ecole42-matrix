use std::error::Error;
use std::fmt;
use std::fmt::Write;
use std::ops;

use num_integer::Roots;
use num_traits::float::Float;
use num_traits::sign::Signed;

pub trait FieldBound:
    Clone
    + PartialEq
    + PartialOrd
    + fmt::Debug
    + fmt::Display
    + ops::Neg<Output = Self>
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    //+ ops::AddAssign
    //+ ops::SubAssign
    //+ ops::MulAssign
    //+ ops::DivAssign
    + for <'a> ops::AddAssign<&'a Self>
    + for <'a> ops::SubAssign<&'a Self>
    + for <'a> ops::MulAssign<&'a Self>
    + for <'a> ops::DivAssign<&'a Self>
{
    const ZERO: Self;
    fn abs(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn is_zero(&self) -> bool;
}

macro_rules! impl_fbound_required {
    ($($t:ty) +, float) => {
        $(
            impl FieldBound for $t {
                const ZERO: Self = 0.0;

                fn abs(&self) -> Self {
                    Signed::abs(self)
                }

                fn sqrt(&self) -> Self {
                    Float::sqrt(self.clone())
                }

                fn is_zero(&self) -> bool {
                    Signed::abs(self) <= 0e-10
                }
            }
        )+
    };

    ($($t:ty) +, integer) => {
        $(
            impl FieldBound for $t {
                const ZERO: Self = 0;

                fn abs(&self) -> Self {
                    Signed::abs(self)
                }

                fn  sqrt(&self) -> Self {
                    Roots::sqrt(self)
                }

                fn is_zero(&self) -> bool {
                    self.clone() == Self::ZERO
                }
            }
        )+
    }
}

impl_fbound_required!(f32 f64, float);
impl_fbound_required!(i8 i16 i32 i64 i128 isize, integer);

#[derive(Debug, PartialEq)]
pub struct D1 {
    length: usize,
}

#[derive(Debug, PartialEq)]
pub struct D2 {
    rows: usize,
    cols: usize,
}

#[derive(Debug, PartialEq)]
pub enum Dimension {
    D1(D1),
    D2(D2),
}

impl Dimension {
    #[allow(dead_code)]
    fn d1(self) -> Option<D1> {
        match self {
            Dimension::D1(d) => Some(d),
            Dimension::D2(_) => None,
        }
    }

    fn d2(self) -> Option<D2> {
        match self {
            Dimension::D1(_) => None,
            Dimension::D2(d) => Some(d),
        }
    }

    #[allow(dead_code)]
    fn inv_eq(&self, other: &Self) -> bool {
        match self {
            Dimension::D1(_) => false,
            Dimension::D2(d2_self) => match other {
                Dimension::D1(_) => false,
                Dimension::D2(d2_other) => {
                    if d2_self.rows == d2_other.cols && d2_self.cols == d2_other.rows {
                        true
                    } else {
                        false
                    }
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct IncompatibleError {
    message: String,
}

impl Error for IncompatibleError {}

impl fmt::Display for IncompatibleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl IncompatibleError {
    fn new(message: String) -> Self {
        IncompatibleError { message }
    }
}

pub trait VectorSpace {
    type Field: FieldBound;

    // Required functions

    // new: automatically fill dimensions with the size of 0
    // fn new(a: &[Self::Field], dims: Dimension) -> Self;

    fn shape(&self) -> Dimension;
    fn size(&self) -> usize;
    // fn reshape(&self, ?) -> ?;

    fn add(&mut self, v: &Self);
    fn sub(&mut self, v: &Self);
    fn scl(&mut self, a: Self::Field);

    fn sum(&self) -> Self::Field;
    fn sqsum(&self) -> Self::Field;

    fn norm_inf(&self) -> Self::Field;
    fn norm_1(&self) -> Self::Field;
    fn norm(&self) -> Self::Field;

    // Provided functions

    fn eq_shape_compatible(&self, v: &Self) -> Result<Dimension, IncompatibleError> {
        if self.shape() == v.shape() {
            Ok(self.shape())
        } else {
            let mut msg = String::new();
            write!(
                &mut msg,
                "received incompatible object of shape {:?}, expected {:?}",
                v.shape(),
                self.shape()
            )
            .unwrap();
            Err(IncompatibleError::new(msg))
        }
    }

    fn eq_size_compatible(&self, v: &Self) -> Result<usize, IncompatibleError> {
        if self.size() == v.size() {
            Ok(self.size())
        } else {
            let mut msg = String::new();
            write!(
                &mut msg,
                "received incompatible object of size {:?}, expected {:?}",
                v.size(),
                self.size()
            )
            .unwrap();
            Err(IncompatibleError::new(msg))
        }
    }

    fn force_eq_shape(&self, v: &Self) {
        match self.eq_shape_compatible(v) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    fn force_eq_size(&self, v: &Self) {
        match self.eq_size_compatible(v) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

pub mod matrix;
pub mod vector;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn dimension_inv_eq_test() {
        let d2_1 = Dimension::D2(D2 { rows: 2, cols: 3 });
        let d2_2 = Dimension::D2(D2 { rows: 3, cols: 2 });

        assert!(d2_1.inv_eq(&d2_2));

        let d1_1 = Dimension::D1(D1 { length: 2 });

        assert!(!d1_1.inv_eq(&d2_2));
    }
}
