use super::Dimension;
use super::FieldBound;
use super::VectorSpace;
use super::D2;

use super::vector::vector;
use super::vector::Vector;

use std::fmt;
use std::ops;
use std::slice;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<K: FieldBound> {
    vectors: Vector<Vector<K>>, // columns
}

fn perfect_square_root(length: usize) -> Option<usize> {
    let root = (length as f64).sqrt().trunc() as usize;
    if root * root == length {
        return Some(root);
    } else {
        return None;
    }
}

macro_rules! is_rectangular {
    ($e: expr) => {
        {
            let len = $e[0].len();
            let mut result = true;
            for i in $e {
                if len != i.size() {
                    result = false;
                    break;
                }
            }
            result
        }
    }
}

#[macro_export]
macro_rules! matrix {
    // TODO: revise initialization without including transpose

    ($([$($e:expr),+]),+) => {
        {
            let mut m = $crate::matrix::Matrix::from(
                $crate::vector::Vector::from(vec![
                    $(
                        $crate::vector::Vector::from(vec![$($e),+]),
                    )+]
                )
            );

            m.transpose();
            m
        }
    };

    ($($e:expr),+) => {
         {
             Matrix::from([$($e),+].as_slice())
         }
    };

    ($e:expr; $c:expr) => {
        {
            $crate::matrix::Matrix::from(
                $crate::vector::vector![
                    $crate::vector::vector![$e; $c];
                    $c
                ]
            )
        }
    };

    ($e: expr; $r:expr, $c:expr) => {
        {
            $crate::matrix::Matrix::from(
                $crate::vector::vector![
                    $crate::vector::vector![$e; $r];
                    $c
                ]
            )
        }
    }
}
pub use matrix;

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

                let mut m = Self {
                    vectors: Vector::from(columns),
                };
                m.transpose();
                m
            }
            None => {
                panic!("array cannot form a squared matrix");
            }
        }
    }
}

impl<K: FieldBound> From<&[Vector<K>]> for Matrix<K> {
    fn from(content: &[Vector<K>]) -> Matrix<K> {
        if !is_rectangular!(content) {
            panic!("array shapes are not uniform");
        }

        let mut m = Self {
            vectors: Vector::from(content),
        };
        m.transpose();

        m
    }
}

impl<K: FieldBound> From<Vector<Vector<K>>> for Matrix<K> {
    fn from(content: Vector<Vector<K>>) -> Matrix<K> {
        Matrix { vectors: content }
    }
}

impl<'a, K: FieldBound> IntoIterator for &'a Matrix<K> {
    type Item = &'a Vector<K>;
    type IntoIter = slice::Iter<'a, Vector<K>>;

    fn into_iter(self) -> Self::IntoIter {
        self.vectors.iter()
    }
}

impl<K: fmt::Display + FieldBound> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // loop over the column and retrieve string repres. of each num
        // with a specific format
        // find the largest string in the column and adjust alignment when
        // printing rows for each member of the column

        let dim: D2 = self.shape().d2().unwrap();
        let mut str_mx = vec![vec![String::from(""); dim.rows]; dim.cols];
        let mut col_len = vec![0; dim.cols];
        for j in 0..dim.cols {
            let j: usize = j;
            for i in 0..dim.rows {
                let i: usize = i;
                str_mx[j][i] = format!("{:.2}", self.vectors[j][i]);
                col_len[j] = col_len[j].max(str_mx[j][i].len());
            }
        }

        let mut str_disp = String::new();
        for i in 0..dim.rows {
            let i: usize = i;
            let mut str_row = String::new();
            for j in 0..dim.cols {
                let j: usize = j;
                let width = col_len[j];
                str_row.push_str(&format!("{:<width$}  ", str_mx[j][i]));
            }
            str_disp.push_str(&str_row);
            str_disp.push_str("\n");
        }

        write!(f, "{}", str_disp)
    }
}

impl<K> std::cmp::PartialOrd for Matrix<K>
where
    K: FieldBound,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.norm().partial_cmp(&other.norm())
    }
}

// MathAssign Implementations

macro_rules! impl_ops_assign {
    ($trait:ty, $fun:ident, $op:tt) => {
        impl<K> $trait for Matrix<K>
        where
            K: FieldBound
        {
            fn $fun(&mut self, rhs: &Self) {
                self.force_eq_shape(&rhs);

                let mut m_iter = rhs.into_iter();
                for col in &mut self.vectors {
                    match m_iter.next() {
                        Some(v_ref) => *col $op v_ref,
                        None => {}
                    }
                }
            }
        }
    }
}

impl_ops_assign!(ops::AddAssign<&Self>, add_assign, +=);
impl_ops_assign!(ops::SubAssign<&Self>, sub_assign, -=);
impl_ops_assign!(ops::MulAssign<&Self>, mul_assign, *=);
impl_ops_assign!(ops::DivAssign<&Self>, div_assign, /=);

impl<K: FieldBound> ops::MulAssign<&Vector<K>> for Matrix<K> {
    // Column-wise multiplication
    fn mul_assign(&mut self, rhs: &Vector<K>) {
        for col in &mut self.vectors {
            *col *= rhs;
        }
    }
}

impl<K: FieldBound> ops::MulAssign<&K> for Matrix<K> {
    fn mul_assign(&mut self, rhs: &K) {
        for col in &mut self.vectors {
            *col *= rhs;
        }
    }
}

impl<K: FieldBound> ops::MulAssign<K> for Matrix<K> {
    fn mul_assign(&mut self, rhs: K) {
        self.mul_assign(&rhs);
    }
}

// VectorSpace Implementation
impl<K: FieldBound> VectorSpace for Matrix<K> {
    type Field = K;

    fn shape(&self) -> Dimension {
        Dimension::D2(D2 {
            rows: self.vectors[0].size(),
            cols: self.vectors.size(),
        })
    }

    fn size(&self) -> usize {
        match self.shape() {
            Dimension::D1(d) => d.length,
            Dimension::D2(d) => d.rows * d.cols,
        }
    }

    fn add(&mut self, v: &Self) {
        *self += v;
    }

    fn sub(&mut self, v: &Self) {
        *self -= v;
    }

    fn scl(&mut self, a: K) {
        *self *= a;
    }

    fn sum(&self) -> K {
        self.vectors.sum().sum()
    }

    fn sqsum(&self) -> K {
        self.vectors.sqsum().sum()
    }

    fn norm_inf(&self) -> K {
        self.vectors.norm_inf().norm_inf()
    }

    fn norm_1(&self) -> K {
        self.vectors.norm_1().sum()
    }

    fn norm(&self) -> K {
        self.sqsum().sqrt()
    }
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

    pub fn append_col(&mut self, v: Vector<K>) {
        self.vectors.append(v);
    }

    pub fn col_sum(&self) -> Vector<K>
    where
        K: FieldBound,
    {
        if self.size() == 0 {
            panic!("Cannot comput col_sum on an empty matrix");
        }

        let k_init = self.vectors[0][0].clone();
        let mut col_sums = vector![k_init; self.vectors.len()];

        for i in 0..self.vectors.len() {
            col_sums[i] = self.vectors[i].sum();
        }

        col_sums
    }

    pub fn mul_vec(&self, v: &Vector<K>) -> Vector<K>
    where
        K: FieldBound,
    {
        let mut m = self.clone();
        m.transpose();
        //ops::MulAssign::mul_assign(self, v); // Cannot use self *= v ?
        m *= v;
        m.col_sum()
    }

    pub fn mul_mat(&self, m: &Matrix<K>) -> Matrix<K>
    where
        K: FieldBound,
    {
        if self.size() == 0 || m.size() == 0 {
            panic!("cannot compute matrix multiplication of an empty matrix");
        }

        let sh_m = m.shape().d2().unwrap();
        let sh_self = self.shape().d2().unwrap();

        if sh_self.cols != sh_m.rows {
            panic!("matrices are incompatible for matrix multiplication");
        }

        let k_init = self.vectors[0][0].clone();
        let inv_n_row = sh_m.cols;
        let inv_n_col = sh_self.rows;
        let mut result = matrix![k_init; inv_n_row, inv_n_col];

        let mut cl = self.clone();
        cl.transpose();

        for j1 in 0..inv_n_col {
            // itr over cols of cl
            for j2 in 0..inv_n_row {
                // itr over cols of m
                result.vectors[j1][j2] = cl.vectors[j1].dot(&m.vectors[j2]);
            }
        }

        result.transpose();
        result
    }
}

// Function Declarations

#[allow(dead_code)]
pub fn lerp<K>(u: &Matrix<K>, v: &Matrix<K>, t: K) -> Matrix<K>
where
    K: FieldBound,
{
    let mut slide = v.clone();
    slide.sub(&u);
    slide.scl(t);

    let mut interp = u.clone();
    interp.add(&slide);

    interp
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matrix_macro_test() {
        let m1 = matrix![[1, 2, 3], [4, 5, 6]];
        let m2 = matrix![1, 2, 3, 4];
        let m3 = matrix![-1; 2];
        let m4 = matrix![0; 3, 2];

        assert_eq!(
            m1,
            Matrix {
                vectors: Vector::from(vec![
                    Vector::from(vec![1, 4]), // 1st column
                    Vector::from(vec![2, 5]), // 2nd column
                    Vector::from(vec![3, 6]), // 3rd column
                ])
            }
        );

        assert_eq!(
            m2,
            Matrix {
                vectors: Vector::from(vec![Vector::from(vec![1, 3]), Vector::from(vec![2, 4]),])
            }
        );

        assert_eq!(
            m3,
            Matrix {
                vectors: Vector::from(
                    vec![Vector::from(vec![-1, -1]), Vector::from(vec![-1, -1]),]
                )
            }
        );

        assert_eq!(
            m4,
            Matrix {
                vectors: Vector::from(vec![
                    Vector::from(vec![0, 0, 0]),
                    Vector::from(vec![0, 0, 0])
                ])
            }
        )
    }

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

        assert_eq!(m1.shape(), Dimension::D2(D2 { rows: 2, cols: 2 }));
        assert_eq!(m2.shape(), Dimension::D2(D2 { rows: 3, cols: 3 }));
        assert_eq!(m3.shape(), Dimension::D2(D2 { rows: 2, cols: 3 }));
    }

    #[test]
    fn size_test() {
        let m1 = Matrix::from([1, 2, 3, 4].as_slice());
        let m2 = Matrix::from([1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice());

        assert_eq!(m1.size(), 4);
        assert_eq!(m2.size(), 9);
    }

    #[test]
    fn col_sum_test() {
        let m = matrix!([1, 2, 3], [4, 5, 6]);

        assert_eq!(m.col_sum(), vector![5, 7, 9]);
    }

    #[test]
    fn add_test() {
        let mut m1 = matrix![1, 2, 3, 4];
        let mut m2 = matrix![[3], [4], [7]];
        let m3 = matrix![2, 2, 2, 2];
        let m4 = matrix![[-1], [3], [0]];

        m1.add(&m3);
        m2.add(&m4);

        assert_eq!(m1, matrix![1 + 2, 2 + 2, 3 + 2, 4 + 2]);
        assert_eq!(m2, matrix![[3 + -1], [4 + 3], [7 + 0]]);
    }

    #[test]
    fn sub_test() {
        let mut m1 = matrix![1, 2, 3, 4];
        let mut m2 = matrix![[3], [4], [7]];
        let m3 = matrix![2, 2, 2, 2];
        let m4 = matrix![[-1], [3], [0]];

        m1.sub(&m3);
        m2.sub(&m4);

        assert_eq!(m1, matrix![1 - 2, 2 - 2, 3 - 2, 4 - 2]);
        assert_eq!(m2, matrix![[3 - -1], [4 - 3], [7 - 0]]);
    }

    #[test]
    fn scl_test() {
        let mut m1 = matrix![1, 2, 3, 4];
        let mut m2 = matrix![[3], [4], [7]];
        let a1 = 2;

        m1.scl(a1);
        m2.scl(a1);

        assert_eq!(m1, matrix![1 * a1, 2 * a1, 3 * a1, 4 * a1]);
        assert_eq!(m2, matrix![[3 * a1], [4 * a1], [7 * a1]]);
    }

    #[test]
    fn lerp_test() {
        let m1 = matrix![[2., 1.], [3., 4.]];
        let m2 = matrix![[20., 10.], [30., 40.]];
        let interp = lerp(&m1, &m2, 0.5);

        assert!(interp > matrix![[10.9, 5.4], [16.4, 21.9]]);
        assert!(interp < matrix![[11.1, 5.6], [16.6, 22.1]]);
    }

    #[test]
    fn mul_vec_test() {
        let u = matrix![[2, -2], [-2, 2]];

        assert_eq!(u.mul_vec(&vector![4, 2]), vector![4, -4]);
    }

    #[test]
    fn mul_mat_test() {
        let u = matrix![[3, -5], [6, 8]];
        let v = matrix![[2, 1], [4, 2]];

        assert_eq!(u.mul_mat(&v), matrix![[-14, -7], [44, 22]]);

        let u = matrix![[1, 2, 3], [4, 5, 6]];
        let v = matrix![[7, 8, 9, 10], [11, 12, 13, 14], [15, 16, 17, 18]];

        assert_eq!(
            u.mul_mat(&v),
            matrix![[74, 80, 86, 92], [173, 188, 203, 218]]
        );
    }
}
