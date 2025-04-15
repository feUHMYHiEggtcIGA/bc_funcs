use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::types::structures_abstr::*;
use bc_utils_lg::enums::indicators::*;
use bc_utils::other;


pub fn bf_sma<T>(
    src: &SRC_ARG<T>,
    window: &usize,
    exc_last: &bool,
) -> FxHashMap<&'static str, Vec<T>>
where 
    T: Float,
{
    let len = src.len();
    let mut src_new = src
        .iter()
        .skip(len - *window - 1)
        .map(Borrow::borrow)
        .collect::<Vec<&T>>();
    if *exc_last {
        other::roll_slice1(&mut src_new, &1);
        FxHashMap::from_iter([
            ("src", src_new.into_iter().skip(1).copied().collect()),
        ])
    } else {
        FxHashMap::from_iter([
            ("src", src_new.into_iter().copied().collect()),
        ])
    }
}

pub fn bf_sma_abstr<'a, T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>,
    exc_last: &bool,
) -> BF_VEC<T>
where 
    T: Float,
{
    vec![T_HASHMAP::VecF(bf_sma(src.first().unwrap(), args.first().unwrap().unwrap_usize(), exc_last))]
}