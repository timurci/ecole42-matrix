pub struct Vector<K> {
    fields: Vec<K>,
}

use super::Dimension;
use super::VectorSpace;
use std::fmt;
use std::ops;

impl<K: Clone> From<Vec<K>> for Vector<K> {
    fn from(content: Vec<K>) -> Vector<K> {
        Vector {
            fields: content.to_vec(),
        }
    }
}

impl<K> VectorSpace for Vector<K>
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

    fn add(&mut self, _v: &Self) {}
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
