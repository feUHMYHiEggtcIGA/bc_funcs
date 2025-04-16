#![allow(non_camel_case_types)]


pub trait AS_SLICE<T> {
    fn as_slice(&self) -> &[T];
}

pub trait AS_ITER<T> {
    fn iter(&self) -> std::slice::Iter<T>;
}