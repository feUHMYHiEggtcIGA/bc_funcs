use std::ops::{
    AddAssign, 
    DivAssign,
};
use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::types::structures::*;

use crate::bf::rma::bf_rma;


pub fn alpha_rma<T: Float>(
    window: &T,
) -> T {
    T::one() / *window
}

pub fn rma<T>(
    src: &T,
    rma_last: &T,
    alpha: &T,
) -> T 
where 
    T: Float,
{
    *alpha * *src + (T::one()- *alpha) * *rma_last
}

#[allow(clippy::implicit_hasher)]
pub fn rma_bf<T: Float>(
    src: &T,
    buff: &mut FxHashMap<&str, T>
) -> T {
    let res = rma(src, &buff["res"], &buff["alpha"]);
    buff.insert("res", res);
    res
}

#[allow(clippy::missing_panics_doc)]
pub fn rma_f<'a, T, V>(
    src: &SRC_ARG<V>,
    window: &usize,
) -> T
where 
    T: Float,
    T: 'a,
    T: AddAssign,
    T: DivAssign,
    V: Borrow<T>,
{
    rma_bf(src.last().unwrap().borrow(), &mut bf_rma(src, window, &true,))
}

#[allow(clippy::needless_borrows_for_generic_args)]
pub fn rma_bf_abstr<T>(
    src: &SRC_ARG<T>,
    _: &ARGS<T>, 
    rm: & mut BF_VEC<T>
) -> T 
where 
    T: Float,
{
    rma_bf(
        src.first().expect("first src not found"),
        rm.first_mut().expect("rm rma not found").unwrap_f(),
    )
}

pub fn rma_f_abstr<T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>,
) -> T
where 
    T: Float,
    T: AddAssign,
    T: DivAssign,
{
    let window = args.first().expect("arg not found").unwrap_usize();
    rma_bf(
        src.first().expect("first src not found").last().unwrap(), 
        &mut bf_rma::<T, T>(src.first().expect("first src not found"), window, &true,)
    )
}

pub fn rma_coll<C, T>(
    src: &SRC_ARG<T>,
    window: &usize,
) -> C 
where 
    T: Float,
    T: AddAssign,
    T: DivAssign,
    C: FromIterator<T>
{
    let w = *window * 10;
    let mut rm = bf_rma::<T, T>(&src[0..=w + 1], window, &true,);
    src
        [w..src.len()]
        .iter()
        .map(
            |v|
            rma_bf(
                v,
                &mut rm
            )
        )
        .collect()
}

pub fn rma_coll_abstr<C, T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>,
) -> C 
where
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    rma_coll(
        src.first().expect("first src not found"),
        args.first().expect("arg window not found in sma_coll_abstr").unwrap_usize()
    )
}
