use std::error::Error;
use std::fmt;
use std::fmt::Write;
use std::ops;

pub trait FieldBound:
    Clone
    + PartialEq
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
{
}

impl FieldBound for f32 {}
impl FieldBound for f64 {}
impl FieldBound for i8 {}
impl FieldBound for i16 {}
impl FieldBound for i32 {}
impl FieldBound for i64 {}
impl FieldBound for i128 {}
impl FieldBound for isize {}

#[derive(Debug, PartialEq, Eq)]
pub enum Dimension {
    D1(usize),
    D2(usize, usize),
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
