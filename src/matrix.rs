use super::Dimension;
use super::FieldBound;
use super::VectorSpace;
use super::D2;

use super::vector::vector;
use super::vector::Vector;

use std::cmp::Ordering;
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

            m.transpose_mut();
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
                m.transpose_mut();
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
        m.transpose_mut();

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
    pub fn transpose_mut(&mut self) {
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

    pub fn transpose(&self) -> Self {
        let mut m = self.clone();
        m.transpose_mut();
        m
    }

    pub fn is_square(&self) -> bool {
        let shape = self.shape().d2().unwrap();
        shape.rows == shape.cols
    }

    pub fn append_col(&mut self, v: Vector<K>) {
        if self.size() != 0 && self.vectors[0].len() != v.len() {
            panic!("cannot append incompatible column");
        }
        self.vectors.append(v);
    }

    fn n_rows(&self) -> usize {
        if self.size() == 0 {
            return 0;
        } else {
            return self.vectors[0].len();
        }
    }

    fn n_cols(&self) -> usize {
        self.vectors.len()
    }

    #[allow(dead_code)]
    fn swap_cols(&mut self, inx1: usize, inx2: usize) {
        let cl1 = self.vectors[inx1].clone();
        self.vectors[inx1] = self.vectors[inx2].clone();
        self.vectors[inx2] = cl1;
    }

    fn swap_rows(&mut self, inx1: usize, inx2: usize) {
        let cl1 = self.clone_row(inx1);
        self.set_row(inx1, &self.clone_row(inx2));
        self.set_row(inx2, &cl1);
    }

    fn clone_row(&self, inx: usize) -> Vector<K> {
        let k_init = self.vectors[0][inx].clone();
        let mut cl = vector![k_init; self.vectors.len()];

        for i in 1..self.vectors.len() {
            cl[i] = self.vectors[i][inx].clone();
        }

        cl
    }

    fn set_row(&mut self, inx: usize, v: &Vector<K>) {
        if v.len() != self.vectors.len() {
            panic!("incorrect row size");
        }

        for i in 0..self.vectors.len() {
            self.vectors[i][inx] = v[i].clone();
        }
    }

    pub fn col_sum(&self) -> Vector<K>
    where
        K: FieldBound,
    {
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
        m.transpose_mut();
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
        cl.transpose_mut();

        for j1 in 0..inv_n_col {
            // itr over cols of cl
            for j2 in 0..inv_n_row {
                // itr over cols of m
                result.vectors[j1][j2] = cl.vectors[j1].dot(&m.vectors[j2]);
            }
        }

        result.transpose_mut();
        result
    }

    pub fn trace(&self) -> K {
        if !self.is_square() {
            panic!("cannot compute trace of a non-square matrix");
        }

        if self.size() == 0 {
            panic!("cannot compute trace of an empty matrix");
        }

        let mut sum = self.vectors[0][0].clone();

        for i in 1..self.vectors.len() {
            sum += &self.vectors[i][i];
        }

        sum
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        // Return reduced row-echelon form

        fn find_nonzero_from<K: FieldBound>(col: &Vector<K>, start: usize) -> usize {
            for i in start..col.len() {
                if !col[i].is_zero() {
                    return i;
                }
            }
            start
        }

        fn div_row_from<K: FieldBound>(m: &mut Matrix<K>, i: usize, from: usize, div: K) {
            for j in from..m.n_cols() {
                m.vectors[j][i] /= &div;
            }
        }

        fn clone_row_from<K: FieldBound>(m: &Matrix<K>, i: usize, from: usize) -> Vector<K> {
            let len = m.vectors.len() - from;
            let mut v = vector![m.vectors[from][i].clone(); len];

            let mut inner_count: usize = 1;
            for j in (from + 1)..m.vectors.len() {
                v[inner_count] = m.vectors[j][i].clone();
                inner_count += 1;
            }

            v
        }

        fn sub_row_from<K>(m: &mut Matrix<K>, i: usize, from: usize, v: Vector<K>)
        where
            K: FieldBound,
        {
            let mut vec_count = 0;
            for j in from..m.n_cols() {
                m.vectors[j][i] -= &v[vec_count];
                vec_count += 1;
            }
        }

        fn neutralize_rows_until<K>(m: &mut Matrix<K>, until: usize, at_col: usize, v: &Vector<K>)
        where
            K: FieldBound,
        {
            for i in 0..until {
                if m.vectors[at_col][i].is_zero() {
                    continue;
                }

                let mut v_scl = v.clone();
                v_scl *= &m.vectors[at_col][i];

                sub_row_from(m, i, at_col, v_scl);
            }
        }

        let mut rech = self.clone();

        let mut i = 0;
        for j in 0..rech.n_cols() {
            // iterate over columns
            let arg = find_nonzero_from(&rech.vectors[j], i);

            if arg != i {
                rech.swap_rows(arg, i);
            }

            if rech.vectors[j][i].is_zero() {
                continue;
            }

            let f = rech.vectors[j][i].clone();
            div_row_from(&mut rech, i, j, f);

            // =================================================
            // Backtracing for reduced row-echelon form
            let partial_row_i = clone_row_from(&rech, i, j);
            neutralize_rows_until(&mut rech, i, j, &partial_row_i);
            // =================================================

            if i == rech.n_rows() - 1 {
                break;
            }

            for i_bp in (i + 1)..rech.n_rows() {
                let f = rech.vectors[j][i_bp].clone();
                let mut scl_part_row_i = partial_row_i.clone();
                scl_part_row_i *= f;
                sub_row_from(&mut rech, i_bp, j, scl_part_row_i);
            }
            i += 1;
        }
        rech
    }

    fn discard(&self, row_index: usize, col_index: usize) -> Matrix<K> {
        let sh_self = self.shape().d2().unwrap();
        let mut m = matrix![self.vectors[0][0].clone(); sh_self.rows - 1, sh_self.cols - 1];
        let sh_m = m.shape().d2().unwrap();

        let mut outer_i: usize = 0;
        for i in 0..sh_m.rows {
            if i == row_index {
                outer_i += 1;
            }

            let mut outer_j: usize = 0;

            for j in 0..sh_m.cols {
                if j == col_index {
                    outer_j += 1;
                }

                m.vectors[j][i] = self.vectors[outer_j][outer_i].clone();

                outer_j += 1;
            }
            outer_i += 1;
        }

        return m;
    }

    fn determinant_sq_lt2(&self) -> K {
        match self.shape().d2().unwrap() {
            D2 { rows: 2, cols: 2 } => {
                return self.vectors[0][0].clone() * self.vectors[1][1].clone()
                    - self.vectors[0][1].clone() * self.vectors[1][0].clone()
            }
            D2 { rows: 1, cols: 1 } => return self.vectors[0][0].clone(),
            _ => panic!("incorrect dimensions"),
        }
    }

    #[allow(dead_code)]
    fn determinant(&self) -> K {
        if !self.is_square() {
            panic!("determinant is not implemented for non-square matrices");
        }
        match self.shape().d2().unwrap().rows.cmp(&2) {
            Ordering::Less => return self.determinant_sq_lt2(),
            Ordering::Equal => return self.determinant_sq_lt2(),
            Ordering::Greater => {
                let mut det: K = self.vectors[0][0].clone();

                for j in 0..self.vectors.len() {
                    let l_det = self.vectors[j][0].clone() * self.discard(0, j).determinant();
                    if j == 0 {
                        det = l_det;
                    } else if j % 2 == 0 {
                        det += &l_det;
                    } else {
                        det -= &l_det;
                    }
                }

                return det;
            }
        }
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

        m1.transpose_mut();
        m2.transpose_mut();

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
    fn swap_cols_test() {
        let mut m = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        m.swap_cols(0, 2);
        assert_eq!(m, matrix![[3, 2, 1], [6, 5, 4], [9, 8, 7]]);
    }

    #[test]
    fn set_row_test() {
        let mut m = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        m.set_row(1, &vector![40, 50, 60]);
        assert_eq!(m, matrix![[1, 2, 3], [40, 50, 60], [7, 8, 9]]);
    }

    #[test]
    fn swap_rows_test() {
        let mut m = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        m.swap_rows(0, 2);
        assert_eq!(m, matrix![[7, 8, 9], [4, 5, 6], [1, 2, 3]]);
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

    #[test]
    fn trace_test() {
        let m = matrix![[2, -5, 0], [4, 3, 7], [-2, 3, 4]];

        assert_eq!(m.trace(), 9);
    }

    #[test]
    fn discard_test() {
        let m = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];

        assert_eq!(m.discard(0, 0), matrix![[5, 6], [8, 9]]);
        assert_eq!(m.discard(0, 1), matrix![[4, 6], [7, 9]]);
        assert_eq!(m.discard(1, 2), matrix![[1, 2], [7, 8]]);
    }

    #[test]
    fn determinant_sq_lt2_test() {
        let m1 = matrix![[2, 7], [5, 9]];
        let m2 = matrix![-3];

        assert_eq!(m1.determinant_sq_lt2(), -17);
        assert_eq!(m2.determinant_sq_lt2(), -3);
    }

    #[test]
    fn determinant_test() {
        let m2 = matrix![[2, 7], [5, 9]];
        let m1 = matrix![-3];
        let m3 = matrix![[1, 22, 3], [30, 51, 16], [7, -8, 5]];
        let m4 = matrix![
            [1, 40, 3, 2],
            [5, 8, 7, 9],
            [19, -7, 60, 4],
            [-13, 12, 17, 24]
        ];
        assert_eq!(m1.determinant(), -3);
        assert_eq!(m2.determinant(), -17);
        assert_eq!(m3.determinant(), -2244);
        assert_eq!(m4.determinant(), -511916);
    }
}
