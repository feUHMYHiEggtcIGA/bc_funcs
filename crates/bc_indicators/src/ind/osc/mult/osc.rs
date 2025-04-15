use std::borrow::Borrow;

use num_traits::Float;
use bc_utils_lg::types::structures_abstr::{ARGS, SRCS_ARG, SRC_ARG};


pub fn mult_osc<T, V>(
    v: V,
    diff_short: &T,
    diff_long: &T,
    max_v_arg: &T,
) -> T
where 
    T: Float,
    V: Borrow<T>,
{
    let diff: T;
    let v2: T;
    let v_b = *v.borrow();
    
    if v_b > (*max_v_arg - *diff_short) {
        diff = *diff_short;
        v2 = *max_v_arg - v_b;
    } else {
        diff = *diff_long;
        v2 = v_b;
    }
    (diff - v2) / diff
}

pub fn mult_osc_abstr<T, V>(
    src: &SRC_ARG<V>,
    args: &ARGS<T>,
) -> T
where
    T: Float,
    V: Borrow<T>,
{
    mult_osc(src[0].borrow(), args[0].unwrap_f(), args[1].unwrap_f(), args[3].unwrap_f())
}

pub fn mult_osc_coll<C, T, V>(
    src: &SRC_ARG<V>,
    diff_short: &T,
    diff_long: &T,
    max_v_arg: &T,
) -> C
where
    T: Float,
    V: Borrow<T>,
    C: FromIterator<T>,
{
    src
        .iter()
        .map(|v| mult_osc(v.borrow(), diff_short, diff_long, max_v_arg))
        .collect()
}

pub fn mult_osc_coll_abstr<C, T, V>(
    src: &SRCS_ARG<V>,
    args: &ARGS<T>
) -> C
where
    T: Float,
    V: Borrow<T>,
    C: FromIterator<T>,
{
    mult_osc_coll::<C, T, V,>(src[0], args[0].unwrap_f(), args[1].unwrap_f(), args[2].unwrap_f())
}