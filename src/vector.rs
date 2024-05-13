#[derive(Debug)]
pub struct Vector<K> {
    fields: Vec<K>,
}

use super::Dimension;
use super::IncompatibleError;
use super::VectorSpace;
use std::fmt;
use std::fmt::Write;
use std::ops;
use std::slice;

impl<K: Clone> From<Vec<K>> for Vector<K> {
    fn from(content: Vec<K>) -> Vector<K> {
        Vector {
            fields: content.to_vec(),
        }
    }
}

impl<'a, K> IntoIterator for &'a Vector<K> {
    type Item = &'a K;
    type IntoIter = slice::Iter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.iter()
    }
}

impl<K: PartialEq> PartialEq for Vector<K> {
    fn eq(&self, other: &Self) -> bool {
        self.fields == other.fields
    }
}

/* // Mutable ref iterator
impl<'a, K> IntoIterator for &'a mut Vector<K> {
    type Item = &'a mut K;
    type IntoIter = slice::IterMut<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.iter_mut()
    }
}
*/

impl<K: Clone> VectorSpace for Vector<K>
where
    K: ops::Add + ops::AddAssign,
{
    type Field = K;

    fn shape(&self) -> Dimension {
        Dimension::D1(self.fields.len())
    }

    fn size(&self) -> usize {
        self.fields.len()
    }

    fn check_compatibility(&self, v: &Self) -> Result<(), IncompatibleError> {
        if self.size() == v.size() {
            Ok(())
        } else {
            let mut msg = String::new();
            write!(
                &mut msg,
                "received incompatible vector of size {}, expected {}",
                v.size(),
                self.size()
            )
            .unwrap();
            Err(IncompatibleError::new(msg))
        }
    }

    fn add(&mut self, v: &Self) {
        self.force_compatibility(v);

        let mut v_iter = v.into_iter();
        for i in &mut self.fields {
            match v_iter.next() {
                Some(k_ref) => *i += k_ref.clone(),
                None => {}
            }
        }
    }
    fn sub(&mut self, _v: &Self) {}
    fn scl(&mut self, _a: K) {}
}

impl<K: fmt::Display> fmt::Display for Vector<K> {
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

// Tests

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add() {
        let mut t1 = Vector::from(vec![1., 2., 3.]);
        let mut t2 = Vector::from(vec![0., -2., 1000., 45.2]);
        let o1 = Vector::from(vec![-5., 3.5, 0.]);
        let o2 = Vector::from(vec![1., 1., 1., 1.]);

        t1.add(&o1);
        t2.add(&o2);

        assert_eq!(t1, Vector::from(vec![-4., 5.5, 3.]));
        assert_eq!(t2, Vector::from(vec![1., -1., 1001., 46.2]));

        t2.add(&o2);

        assert_eq!(t2, Vector::from(vec![2., 0., 1002., 47.2]));
    }

    #[test]
    fn check_compatibility() {
        let t1 = Vector::from(vec![1., 2., 3.]);
        let t2 = Vector::from(vec![0., -2., 1000., 45.2]);
        let t3 = Vector::from(vec![100., -32932.3, 42124.6, 0.]);

        assert!(t1.check_compatibility(&t2).is_err());
        assert!(matches!(t3.check_compatibility(&t2), Ok(_)));
    }

    #[test]
    #[should_panic]
    fn force_compatbility() {
        let t1 = Vector::from(vec![1., 2., 3.]);
        let t2 = Vector::from(vec![0., -2., 1000., 45.2]);

        t1.force_compatibility(&t2);
    }
}
