use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;

use crate::indicators::no_oscillators::trend;
use crate::rm;

#[allow(clippy::missing_panics_doc)]
pub fn rsi<'a, T>(
    rma1: &T,
    rma2: &T,
) -> T
where 
    T: 'a,
    T: Float,
{
    let one_h = T::from(100.0).unwrap();

    one_h - (one_h / (T::one() + *rma1 / *rma2))
}

#[allow(clippy::implicit_hasher)]
pub fn rsi_rm<T>(
    src: &T,
    rm: &mut FxHashMap<&'static str, T>,
    rm_rma1: &mut FxHashMap<&'static str, T>,
    rm_rma2: &mut FxHashMap<&'static str, T>,
) -> T
where 
    T: Float,
{
    let change = *src - rm["src"];
    let u = T::zero().max(change);
    let d = T::zero().max(-change);
    
    let rma1 = trend::rma_rm(&u, rm_rma1);
    let rma2 = trend::rma_rm(&d, rm_rma2);
    let res = rsi(&rma1, &rma2);
    rm.insert("src", *src);
    rm_rma1.insert("res", rma1);
    rm_rma2.insert("res", rma2);
    res
}

#[allow(clippy::missing_panics_doc)]
pub fn rsi_float<T, V>(
    src: &[V],
    window: &usize,
) -> T
where 
    T: Float, 
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    V: Borrow<T>,
{
    let (
        mut rm,
        mut rm_rma1,
        mut rm_rma2,
    ) = rm::rm_rsi(src, window);
    rsi_rm(
        src.last().unwrap().borrow(), 
        &mut rm, 
        &mut rm_rma1, 
        &mut rm_rma2
    )
}
