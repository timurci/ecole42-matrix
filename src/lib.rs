pub enum Dimension {
    D1(usize),
    D2(usize, usize),
}

use std::error::Error;
use std::fmt;

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
    type Field;

    // Required functions

    // new: automatically fill dimensions with the size of 0
    // fn new(a: &[Self::Field], dims: Dimension) -> Self;

    fn check_compatibility(&self, v: &Self) -> Result<(), IncompatibleError>;

    fn shape(&self) -> Dimension;
    fn size(&self) -> usize;
    // fn reshape(&self, ?) -> ?;

    fn add(&mut self, v: &Self);
    fn sub(&mut self, v: &Self);
    fn scl(&mut self, a: Self::Field);

    // Implemented functions

    fn force_compatibility(&self, v: &Self) {
        match self.check_compatibility(v) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

pub mod matrix;
pub mod vector;
