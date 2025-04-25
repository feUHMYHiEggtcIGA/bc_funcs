use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::structs_and_types::structures_abstr::*;
use bc_utils_lg::enums::indicators::*;

use crate::ind::no_osc::trend::rma::*;

#[allow(clippy::missing_panics_doc)]
pub fn bf_rma<'a, T, V>(
    src: &SRC_ARG<V>,
    window: &usize,
    exc_last: &bool,
) -> FxHashMap<&'static str, T>
where 
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    V: Borrow<T>,
{
    let mut res = T::zero();
    let len = src.len();
    let window_t = T::from(*window).unwrap();
    
    let alpha = alpha_rma(&window_t);
    for (i, el) in {
        if *exc_last {&src[len - *window * 10 - 1..len - 1]}
        else {&src[len - *window * 10..len]}
    }
        .iter()
        .enumerate() 
    {
        if i < *window {
            res += *el.borrow();
            continue;
        }
        if i == *window - 1{
            res /= window_t;
        }
        res = rma(
            el.borrow(), 
            &res, 
            &alpha,
        );
    }
    FxHashMap::from_iter([
        ("alpha", alpha),
        ("res", res
        ),
    ])
}

pub fn bf_rma_abstr<'a, T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T, T>,
    exc_last: &bool,
) -> BF_VEC<T>
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    vec![T_HASHMAP::Float(bf_rma::<T, T>(&src[0], args.first().unwrap().unwrap_usize(), exc_last))]
}