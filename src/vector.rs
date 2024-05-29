use super::Dimension;
use super::FieldBound;
use super::VectorSpace;
use super::D1;
use std::fmt;
use std::ops;
use std::slice;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vector<K: FieldBound> {
    fields: Vec<K>,
}

#[macro_export]
macro_rules! vector {
    ($($e:expr),+) => {
        $crate::vector::Vector::from(vec![$($e),+])
    };

    ($e:expr; $c:expr) => {
        $crate::vector::Vector::from(vec![$e; $c])
    }
}
pub use vector;

impl<K: FieldBound> From<&[K]> for Vector<K> {
    fn from(content: &[K]) -> Vector<K> {
        Vector {
            fields: content.to_vec(),
        }
    }
}

impl<K: FieldBound> From<Vec<K>> for Vector<K> {
    fn from(content: Vec<K>) -> Vector<K> {
        Vector {
            fields: content.to_vec(),
        }
    }
}

impl<'a, K: FieldBound> IntoIterator for &'a Vector<K> {
    type Item = &'a K;
    type IntoIter = slice::Iter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Mutable ref iterator
impl<'a, K: FieldBound> IntoIterator for &'a mut Vector<K> {
    type Item = &'a mut K;
    type IntoIter = slice::IterMut<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.iter_mut()
    }
}

impl<K: FieldBound> ops::Index<usize> for Vector<K> {
    type Output = K;
    fn index(&self, i: usize) -> &K {
        &self.fields[i]
    }
}

impl<K: FieldBound> ops::IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, i: usize) -> &mut K {
        &mut self.fields[i]
    }
}

impl<K: fmt::Display + FieldBound> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut content = String::from("[");
        for num in &self.fields[..] {
            content.push_str(&num.to_string());
            content.push_str(", ");
        }
        match content.rfind(", ") {
            Some(i) => content.replace_range(i..content.len(), ""),
            None => {}
        }
        content.push_str("]");
        write!(f, "{}", content)
    }
}

// FieldBound Implementation

impl<K> ops::AddAssign<&Self> for Vector<K>
where
    K: FieldBound, //+ for<'a> ops::AddAssign<&'a K>,
{
    fn add_assign(&mut self, rhs: &Self) {
        self.force_eq_size(&rhs);

        let mut v_iter = rhs.into_iter();
        for i in &mut self.fields {
            match v_iter.next() {
                Some(k_ref) => *i += k_ref,
                None => {}
            }
        }
    }
}

impl<K> ops::SubAssign<&Self> for Vector<K>
where
    K: FieldBound,
{
    fn sub_assign(&mut self, rhs: &Self) {
        self.force_eq_size(&rhs);

        let mut v_iter = rhs.into_iter();
        for i in &mut self.fields {
            match v_iter.next() {
                Some(k_ref) => *i -= k_ref,
                None => {}
            }
        }
    }
}

impl<K> ops::MulAssign<&Self> for Vector<K>
where
    K: FieldBound,
{
    fn mul_assign(&mut self, rhs: &Self) {
        self.force_eq_size(&rhs);

        let mut v_iter = rhs.into_iter();
        for i in &mut self.fields {
            match v_iter.next() {
                Some(k_ref) => *i *= k_ref,
                None => {}
            }
        }
    }
}

impl<K> ops::DivAssign<&Self> for Vector<K>
where
    K: FieldBound,
{
    fn div_assign(&mut self, rhs: &Self) {
        self.force_eq_size(&rhs);

        let mut v_iter = rhs.into_iter();
        for i in &mut self.fields {
            match v_iter.next() {
                Some(k_ref) => *i *= k_ref,
                None => {}
            }
        }
    }
}

impl<K: FieldBound> FieldBound for Vector<K> {}

// Independent MulAssign Implementation

impl<K> ops::MulAssign<&K> for Vector<K>
where
    K: FieldBound,
{
    fn mul_assign(&mut self, rhs: &K) {
        for i in &mut self.fields {
            *i *= rhs;
        }
    }
}

impl<K> ops::MulAssign<K> for Vector<K>
where
    K: FieldBound,
{
    fn mul_assign(&mut self, rhs: K) {
        self.mul_assign(&rhs);
    }
}

// VectorSpace Implementation

impl<K: FieldBound> VectorSpace for Vector<K> {
    type Field = K;

    fn shape(&self) -> Dimension {
        Dimension::D1(D1 {
            length: self.fields.len(),
        })
    }

    fn size(&self) -> usize {
        self.fields.len()
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
}

impl<K: FieldBound> Vector<K> {
    pub fn len(&self) -> usize {
        self.size()
    }

    pub fn iter(&self) -> slice::Iter<'_, K> {
        self.fields.iter()
    }

    pub fn sum(&self) -> K {
        let mut sum: K = self.fields[0].clone();

        for i in 1..self.len() {
            sum += &self.fields[i];
        }

        sum
    }

    pub fn sqsum(&self) -> K {
        let mut sqsum: K = self.fields[0].clone();
        sqsum *= &sqsum.clone();

        for i in 1..self.len() {
            let mut field = self.fields[i].clone();
            field *= &field.clone();
            sqsum += &field;
        }

        sqsum
    }

    pub fn dot(&self, v: &Vector<K>) -> K {
        let mut c = self.clone();
        c *= v;
        c.sum()
    }
}

// TODO: Consider moving previous functions into macros

macro_rules! norm_impl {
    ($($t:ty) +, float) => {
        $(
            impl Vector<$t> {
                pub fn norm(&self) -> $t {
                    self.sqsum().sqrt()
                }

                pub fn norm_inf(&self) -> $t {
                    let mut max = self.fields[0].abs();

                    for field in self {
                        if field.abs() > max {
                            max = field.abs();
                        }
                    }

                    max
                }
            }
        )+
    };

    ($($t:ty) +, integer) => {
        $(
            impl Vector<$t> {
                pub fn norm(&self) -> $t {
                    let sqsum = self.sqsum();
                    (sqsum as f64).sqrt() as $t
                }

                pub fn norm_inf(&self) -> $t {
                    let mut max = self.fields[0].abs();

                    for field in self {
                        if field.abs() > max {
                            max = field.abs();
                        }
                    }

                    max
                }
            }
        )+
    }
}

norm_impl!(f32 f64, float);
norm_impl!(i8 i16 i32 i64 i128 isize, integer);

macro_rules! norm_1_impl {
    ($($t:ty) +) => {
        $(
            impl Vector<$t> {
                pub fn norm_1(&self) -> $t {
                    let mut sum = self.fields[0].clone().abs();

                    for i in 1..self.len() {
                        sum += self.fields[i].abs();
                    }

                    sum
                }
            }
        )+
    };
}

norm_1_impl!(f32 f64 i8 i16 i32 i64 i128 isize);

// Function Declarations

#[allow(dead_code)]
pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: FieldBound,
{
    let mut vsum = u[0].clone();

    vsum *= &coefs[0];

    for i in 1..u.len() {
        let mut v = u[i].clone();

        v *= &coefs[i];
        vsum.add(&v);
    }

    vsum
}

#[allow(dead_code)]
pub fn lerp<K>(u: &Vector<K>, v: &Vector<K>, t: K) -> Vector<K>
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

/*
   TODO: consider using num crate to be able to declare functions outside of
         impl block
*/
/*
#[allow(dead_code)]
pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> K
where
    K: FieldBound,
{
    let mut acos = u.dot(v);
    acos /= &u.norm();
    acos /= &v.norm();

    acos
}
*/

// Tests

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn vector_macro_test() {
        let v1 = vector![1, 5, -2, 3];
        let v2 = vector![924; 5];

        assert_eq!(
            v1,
            Vector {
                fields: vec![1, 5, -2, 3]
            }
        );
        assert_eq!(
            v2,
            Vector {
                fields: vec![924, 924, 924, 924, 924]
            }
        );
    }

    #[test]
    fn add_test() {
        let mut t1 = Vector::from(vec![1, 2, 3]);
        let mut t2 = Vector::from([0, -2, 1000, 45].as_slice());
        let o1 = Vector::from(vec![-5, 3, 0]);
        let o2 = Vector::from(vec![1, 1, 1, 1]);

        t1.add(&o1);
        t2.add(&o2);

        assert_eq!(t1, Vector::from(vec![-4, 5, 3]));
        assert_eq!(t2, Vector::from(vec![1, -1, 1001, 46]));

        t1 += &o1;
        t2.add(&o2);

        assert_eq!(t1, Vector::from(vec![-9, 8, 3]));
        assert_eq!(t2, Vector::from(vec![2, 0, 1002, 47]));
    }

    #[test]
    fn sub_test() {
        let mut t1 = Vector::from(vec![-9, 9, 3]);
        let o1 = Vector::from(vec![-5, 3, 0]);

        t1.sub(&o1);
        assert_eq!(t1, Vector::from(vec![-4, 6, 3]));

        t1 -= &o1;
        assert_eq!(t1, Vector::from(vec![1, 3, 3]));
    }

    #[test]
    fn vector_multiplication_test() {
        let mut t1 = Vector::from(vec![1, 2, 3]);
        let o1 = Vector::from(vec![-5, 3, 21]);

        t1 *= &o1;
        assert_eq!(t1, Vector::from(vec![-5, 6, 63]));
    }

    #[test]
    fn scalar_multiplication_test() {
        let mut t1 = Vector::from(vec![1, 2, 3]);
        let s1 = 25;

        t1 *= s1;
        assert_eq!(t1, Vector::from(vec![25, 50, 75]));

        t1.scl(s1);
        assert_eq!(t1, Vector::from(vec![625, 1250, 1875]));
    }

    #[test]
    fn check_compatibility_test() {
        let t1 = Vector::from(vec![1, 2, 3]);
        let t2 = Vector::from(vec![0, -2, 1000, 45]);
        let t3 = Vector::from(vec![100, -32932, 42124, 0]);

        assert!(t1.eq_size_compatible(&t2).is_err());
        assert!(matches!(t3.eq_shape_compatible(&t2), Ok(_)));
    }

    #[test]
    fn linear_combination_test() {
        let v1 = vector![1, 2, 3];
        let v2 = vector![0, 10, -100];

        assert_eq!(
            vector![10, 0, 230],
            linear_combination([v1, v2].as_slice(), [10, -2].as_slice())
        )
    }

    #[test]
    fn lerp_test() {
        let interp = lerp(&vector![2., 1.], &vector![4., 2.], 0.3);

        assert!(interp > vector![2.5, 1.2]);
        assert!(interp < vector![2.7, 1.4]);
    }

    #[test]
    fn dot_test() {
        assert_eq!(vector![-1, 6].dot(&vector![3, 2]), 9);
    }

    #[test]
    fn norm_1_test() {
        let v1: Vector<i32> = vector![1, 2, 3];
        let v2: Vector<i16> = vector![-1, -2];

        assert_eq!(v1.norm_1(), 6);
        assert_eq!(v2.norm_1(), 3);
    }

    #[test]
    fn norm_test() {
        let v1: Vector<f32> = vector![1., 2., 3.];

        assert!(v1.norm() > 3.740);
        assert!(v1.norm() < 3.742);
    }

    #[test]
    fn norm_inf_test() {
        let v1: Vector<i32> = vector![1, 2, 3];
        let v2: Vector<i16> = vector![-1, -2];

        assert_eq!(v1.norm_inf(), 3);
        assert_eq!(v2.norm_inf(), 2);
    }

    #[test]
    #[should_panic]
    fn force_compatibility_test() {
        let t1 = Vector::from(vec![1, 2, 3]);
        let t2 = Vector::from(vec![0, -2, 1000, 45]);

        t1.force_eq_size(&t2);
    }
}
