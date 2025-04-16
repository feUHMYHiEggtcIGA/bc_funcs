#![allow(non_camel_case_types)]

use crate::traits::coll::*;


pub struct VEC_F64(pub Vec<f64>);

impl IntoIterator for VEC_F64 {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<f64>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Clone for VEC_F64 {
    fn clone(&self) -> Self {
        VEC_F64(self.0.clone())
    }
}

impl FromIterator<f64> for VEC_F64 {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        VEC_F64(iter.into_iter().collect())
    }
}

impl AS_SLICE<f64> for VEC_F64 {
    fn as_slice(&self) -> &[f64] {
        self.0.as_slice()
    }
}

impl AS_ITER<f64> for VEC_F64 {
    fn iter(&self) -> std::slice::Iter<f64> {
        self.0.iter()
    }
}