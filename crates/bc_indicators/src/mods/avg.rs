use std::borrow::Borrow;

use num_traits::Float;
use bc_utils_lg::types::structures::{ARGS, SRC_ARG, SRCS_ARG, BF_VEC};
use bc_utils::nums::avg_with;


pub fn avg_abstr<T>(
    v: &T,
    add: &SRC_ARG<T>,
    _: &ARGS<T>, 
) -> T
where 
    T: Float,
    T: std::ops::AddAssign,
{
    avg_with(v, add)
}

pub fn avg_from_coll_abstr<T>(
    v: &SRC_ARG<T>,
    add: &SRCS_ARG<T>,
    _: &ARGS<T>
) -> T
where 
    T: Float,
    T: std::ops::AddAssign,
{
    avg_with(&v[0], add[0])
}

pub fn avg_bf_abstr<T>(
    v: &T,
    add: &SRC_ARG<T>,
    _: &ARGS<T>, 
    _: &mut BF_VEC<T>
) -> T
where 
    T: Float,
    T: std::ops::AddAssign,
{
    avg_with(v, add)
}

pub fn avg_coll<C, T>(
    src: &SRC_ARG<T>,
    add: &SRCS_ARG<T>,
) -> C
where 
    T: Float,
    T: std::iter::Sum,
    T: std::ops::AddAssign,
    C: FromIterator<T>,
{
    // change this pizdec
    src
        .iter()
        .enumerate()
        .map(
            |(i, v)| avg_with(
                v.borrow(),
                add
                    .iter()
                    .map(|v| v[i])
                    .collect::<Vec<T>>()
                    .as_slice()
            )
        )
        .collect()
}

pub fn avg_coll_abstr<C, T>(
    src: &SRC_ARG<T>,
    add: &SRCS_ARG<T>,
    _: &ARGS<T>,
) -> C
where 
    T: Float,
    T: std::iter::Sum,
    T: std::ops::AddAssign,
    C: FromIterator<T>,
{
    avg_coll(src, add)
}