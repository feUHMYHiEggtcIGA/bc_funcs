use std::borrow::Borrow;

use num_traits::Float;
use bc_utils_lg::structs_and_types::structures_abstr::*;


pub fn percent<T, V>(
    now: V,
    past: V, 
) -> T 
where
    T: Float,
    V: Borrow<T>,
    V: Copy,
{
    let past = *past.borrow();
    (*now.borrow() - past) / past
}

pub fn percent_abstr<T, V>(
    src: &SRC_ARG<T>,
    _: &ARGS<T, T>,
) -> T 
where
    T: Float,
    V: Borrow<T>,
    V: Copy,
{
    percent(&src[0], &src[1])
}

pub fn percent_coll<C, T, V>(
    src1: &SRC_ARG<T>,
    src2: &SRC_ARG<T>,
) -> C
where
    T: Float,
    V: Borrow<T>,
    V: Copy,
    C: FromIterator<T>,
{
    src1
        .iter()
        .zip(src2)
        .map(|(v1, v2)| percent(v1, v2))
        .collect()
}

pub fn percent_coll_abstr<C, T, V>(
    src: &SRCS_ARG<T>,
    _: ARGS<T, T>,
) -> C
where 
    T: Float,
    V: Borrow<T>,
    V: Copy,
    C: FromIterator<T>,
{
    percent_coll::<C, T, V>(src[0], src[1])
}