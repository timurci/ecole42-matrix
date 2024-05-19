use super::Dimension;
use super::FieldBound;
use super::VectorSpace;

use super::vector::Vector;

fn perfect_square_root(length: usize) -> Option<usize> {
    let root = (length as f64).sqrt().trunc() as usize;
    if root * root == length {
        return Some(root);
    } else {
        return None;
    }
}

fn is_rectangular<K: FieldBound>(content: &[Vector<K>]) -> bool {
    let len = content[0].size();
    for i in content {
        if len != i.size() {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone)]
pub struct Matrix<K: FieldBound> {
    vectors: Vector<Vector<K>>,
}

impl<K: FieldBound> From<&[K]> for Matrix<K> {
    fn from(content: &[K]) -> Matrix<K> {
        match perfect_square_root(content.len()) {
            Some(root) => {
                let v = content[0].clone();
                let col = Vector::from([v].as_slice());
                let mut columns = vec![col; root];
                let mut counter: usize = 0;

                for item in &mut columns {
                    *item = Vector::from(&content[(counter * root)..((counter + 1) * root)]);
                    counter += 1;
                }

                Self {
                    vectors: Vector::from(columns),
                }
            }
            None => {
                panic!("array cannot form a squared matrix");
            }
        }
    }
}

impl<K: FieldBound> From<&[Vector<K>]> for Matrix<K> {
    fn from(content: &[Vector<K>]) -> Matrix<K> {
        if !is_rectangular(content) {
            panic!("array shapes are not uniform");
        }

        let mut m = Self {
            vectors: Vector::from(content),
        };
        m.transpose();

        m
    }
}

impl<K: FieldBound> PartialEq for Matrix<K> {
    fn eq(&self, other: &Self) -> bool {
        self.vectors == other.vectors
    }
}

//impl<K: fmt::Display + FieldBound> fmt::Display for Matrix<K> {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//
//    }
//}

impl<K: FieldBound> VectorSpace for Matrix<K> {
    type Field = K;

    fn shape(&self) -> Dimension {
        Dimension::D2(self.vectors[0].size(), self.vectors.size())
    }

    fn size(&self) -> usize {
        match self.shape() {
            Dimension::D1(s) => s,
            Dimension::D2(r, c) => r * c,
        }
    }

    fn add(&mut self, _v: &Self) {}

    fn sub(&mut self, _v: &Self) {}

    fn scl(&mut self, _a: K) {}
}

impl<K: FieldBound> Matrix<K> {
    pub fn transpose(&mut self) {
        let pivot = self.vectors.clone();
        self.vectors = Vector::from(vec![pivot[0].clone(); pivot[0].size()]);
        for i in 0..pivot[0].size() {
            let mut row = vec![pivot[0][0].clone(); pivot.size()];
            for j in 0..pivot.size() {
                row[j] = pivot[j][i].clone();
            }
            self.vectors[i] = Vector::from(row);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn perfect_square_root_test() {
        let x: usize = 50;
        let y: usize = 49;

        assert_eq!(perfect_square_root(x), None);
        assert_eq!(perfect_square_root(y), Some(7 as usize));
    }

    #[test]
    fn transpose_test() {
        let mut m1 = Matrix::from([1, 2, 3, 4].as_slice());
        let mut m2 = Matrix::from([1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice());

        m1.transpose();
        m2.transpose();

        assert_eq!(m1, Matrix::from([1, 3, 2, 4].as_slice()));
        assert_eq!(m2, Matrix::from([1, 4, 7, 2, 5, 8, 3, 6, 9].as_slice()));
    }

    #[test]
    fn shape_test() {
        let m1 = Matrix::from([1, 2, 3, 4].as_slice());
        let m2 = Matrix::from([1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice());
        let m3: Matrix<i32> =
            Matrix::from([Vector::from(vec![1, 2, 3]), Vector::from(vec![3, 4, 5])].as_slice());

        assert_eq!(m1.shape(), Dimension::D2(2, 2));
        assert_eq!(m2.shape(), Dimension::D2(3, 3));
        assert_eq!(m3.shape(), Dimension::D2(2, 3));
    }

    #[test]
    fn size_test() {
        let m1 = Matrix::from([1, 2, 3, 4].as_slice());
        let m2 = Matrix::from([1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice());

        assert_eq!(m1.size(), 4);
        assert_eq!(m2.size(), 9);
    }
}
