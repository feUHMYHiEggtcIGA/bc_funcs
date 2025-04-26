use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::types::structures::*;
use bc_utils_lg::enums::indicators::*;

use crate::bf::rma::bf_rma;


#[allow(clippy::implicit_hasher)]
#[allow(clippy::type_complexity)]
pub fn bf_rsi<'a, T, V>(
    src: &SRC_ARG<V>,
    window: &usize,
    exc_last: &bool,
) -> (
    FxHashMap<&'static str, T>,
    FxHashMap<&'static str, T>, 
    FxHashMap<&'static str, T>,
)
where 
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    V: Borrow<T>,
{
    let mut u = Vec::new();
    let mut d = Vec::new();
    let mut src_l = T::nan();
    let len_src = src.len();
    let _w = *window * 10;

    for (i, el) in {
        if *exc_last {&src[len_src - _w - 2..len_src - 1]}
        else {&src[len_src - _w - 1..]}
    }
        .iter()
        .enumerate()
    {
        if i == 0 {
            src_l = *el.borrow();
            continue;
        }
        let change = *el.borrow() - src_l;
        u.push(change.max(T::zero()));
        d.push((-change).max(T::zero()));
        src_l = *el.borrow();
    }
    (
        FxHashMap::from_iter([("src", src_l)]),
        bf_rma::<T, T>(u.as_slice(), window, &false),
        bf_rma::<T, T>(d.as_slice(), window, &false)
    )
}

pub fn bf_rsi_abstr<T, V>(
    src: &SRCS_ARG<V>,
    args: &ARGS<T>,
    exc_last: &bool,
) -> BF_VEC<T>
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    V: Borrow<T>,
{
    let rm = bf_rsi(
        src[0],
        args[0].unwrap_usize(), 
        exc_last, 
    );
    vec![
        T_HASHMAP::Float(rm.0),
        T_HASHMAP::Float(rm.1),
        T_HASHMAP::Float(rm.2),
    ]
}