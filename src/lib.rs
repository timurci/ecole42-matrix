pub enum Dimension {
    D1(usize),
    D2(usize, usize),
}

pub trait VectorSpace {
    type Field;

    // new: automatically fill dimensions with the size of 0
    //fn new(a: &[Self::Field], dims: Dimension) -> Self;

    fn shape(&self) -> Dimension;
    fn size(&self) -> usize;
    // fn reshape(&self, ?) -> ?;

    fn add(&mut self, v: &Self);
    fn sub(&mut self, v: &Self);
    fn scl(&mut self, a: Self::Field);
}

pub mod matrix;
pub mod vector;
