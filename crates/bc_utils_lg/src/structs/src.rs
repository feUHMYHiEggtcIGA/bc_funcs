#![allow(non_camel_case_types)]

use num_traits::Float;


#[derive(Debug, Copy, Clone)]
pub struct SRC_EL<T>
where
    T: Float,
{
    pub open: T,
    pub high: T,
    pub low: T,
    pub close: T,
}

#[derive(Debug, Copy, Clone)]
pub struct SRC_EL_T<T>
where
    T: Float,
{
    pub open: T,
    pub high: T,
    pub low: T,
    pub close: T,
    pub time: T,
}

#[derive(Debug, Copy, Clone)]
pub struct SRC_EL_TT<T> 
where
    T: Float,
{
    pub open: T,
    pub high: T,
    pub low: T,
    pub close: T,
    pub time: T,
    pub volume: T,
}

#[derive(Debug, Copy, Clone)]
pub struct SRC_EL_TVT<T>
where
    T: Float,
{
    pub open: T,
    pub high: T,
    pub low: T,
    pub close: T,
    pub time: T,
    pub volume: T,
    pub turnover: T,
}

#[derive(Debug, Copy, Clone)]
pub struct SRC<'a, T>
where
    T: Float,
{
    pub open: &'a [T],
    pub high: &'a [T],
    pub low: &'a [T],
    pub close: &'a [T],
}