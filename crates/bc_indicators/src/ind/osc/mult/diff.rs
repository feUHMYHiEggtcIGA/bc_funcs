use std::borrow::Borrow;

use num_traits::Float;
use bc_utils_lg::types::structures::{ARGS, SRCS_ARG, SRC_ARG};

pub fn mult_diff<T, V>(
    v1: V,
    v2: V,
    multiplier: &T,
) -> T 
where 
    T: Float,
    V: Borrow<T>
{
    let min_ = v1.borrow().min(*v2.borrow());
    let max_ = v1.borrow().max(*v2.borrow());

    let diff = ((max_ / min_) - T::one()) * *multiplier;
    if diff > T::one() {
        return T::one();
    }
    diff
}

pub fn mult_diff_abstr<T, V>(
    src: &SRC_ARG<V>,
    args: &ARGS<T>,
) -> T
where 
    T: Float,
    V: Borrow<T>
{
    mult_diff(src[0].borrow(), src[1].borrow(), args[0].unwrap_f())
}

pub fn mult_diff_coll<C, T, V>(
    src1: &SRC_ARG<V>,
    src2: &SRC_ARG<V>,
    multiplier: &T,
) -> C
where 
    T: Float,
    V: Borrow<T>,
    C: FromIterator<T>,
{
    src1
        .iter()
        .zip(src2)
        .map(|(v1, v2)| mult_diff(v1.borrow(), v2.borrow(), multiplier))
        .collect()
}

pub fn mult_diff_coll_abstr<C, T, V>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>
) -> C
where 
    T: Float,
    V: Borrow<T>,
    C: FromIterator<T>,
{
    mult_diff_coll(src[0], src[1], args[0].unwrap_f())
}