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
    + ops::Neg<Output = Self>
    //+ ops::Add
    //+ ops::Sub
    //+ ops::Mul
    //+ ops::Div
    //+ ops::Neg
    //+ ops::AddAssign
    //+ ops::SubAssign
    //+ ops::MulAssign
    //+ ops::DivAssign
    + for <'a> ops::AddAssign<&'a Self>
    + for <'a> ops::SubAssign<&'a Self>
    + for <'a> ops::MulAssign<&'a Self>
    + for <'a> ops::DivAssign<&'a Self>
{
    fn abs(&self) -> Self;
    fn sqrt(&self) -> Self;
}

macro_rules! impl_fbound_required {
    ($($t:ty) +, float) => {
        $(
            impl FieldBound for $t {
                fn abs(&self) -> Self {
                    Signed::abs(self)
                }

                fn sqrt(&self) -> Self {
                    Float::sqrt(self.clone())
                }
            }
        )+
    };

    ($($t:ty) +, integer) => {
        $(
            impl FieldBound for $t {
                fn abs(&self) -> Self {
                    Signed::abs(self)
                }

                fn  sqrt(&self) -> Self {
                    Roots::sqrt(self)
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
