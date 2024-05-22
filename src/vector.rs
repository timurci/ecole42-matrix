use super::Dimension;
use super::FieldBound;
use super::VectorSpace;
use super::D1;
use std::fmt;
use std::ops;
use std::slice;

#[derive(Debug, Clone)]
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

impl<K: FieldBound> PartialEq for Vector<K> {
    fn eq(&self, other: &Self) -> bool {
        self.fields == other.fields
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
}

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
    #[should_panic]
    fn force_compatibility_test() {
        let t1 = Vector::from(vec![1, 2, 3]);
        let t2 = Vector::from(vec![0, -2, 1000, 45]);

        t1.force_eq_size(&t2);
    }
}
