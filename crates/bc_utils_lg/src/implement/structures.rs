#![allow(non_camel_case_types)]

use crate::traits::coll::*;


pub struct VEC<T>(pub Vec<T>);

impl<T> IntoIterator for VEC<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Clone for VEC<T> 
where
    T: Clone
{
    fn clone(&self) -> Self {
        VEC(self.0.clone())
    }
}

impl<T> FromIterator<T> for VEC<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        VEC(iter.into_iter().collect())
    }
}

impl<T> AS_SLICE<T> for VEC<T> {
    fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T> AS_ITER<T> for VEC<T> {
    fn iter(&self) -> std::slice::Iter<T> {
        self.0.iter()
    }
}