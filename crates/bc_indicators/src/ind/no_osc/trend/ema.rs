use std::ops::{
    AddAssign, 
    DivAssign,
};
use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::types::structures::*;

use crate::bf::ema::bf_ema;


#[allow(clippy::missing_panics_doc)]
pub fn alpha_ema<T>(
    window: &T,
) -> T 
where
    T: Float,
{
    T::from(2.0).unwrap() / (*window + T::one())
}

pub fn ema<T>(
    src: &T,
    ema_last: &T,
    alpha: &T,
) -> T 
where 
    T: Float,
{
    (*src.borrow() * *alpha) + (*ema_last.borrow() * (T::one() - *alpha))
}

#[allow(clippy::implicit_hasher)]
pub fn ema_bf<T>(
    src: &T,
    buff: &mut FxHashMap<&str, T>
) -> T 
where 
    T: Float,
{
    let res = ema(src, &buff["res"], &buff["alpha"]);
    buff.insert("res", res);
    res
}

#[allow(clippy::needless_borrows_for_generic_args)]
pub fn ema_bf_abstr<T>(
    src: &SRC_ARG<T>,
    _: &ARGS<T>, 
    bf: &mut BF_VEC<T>
) -> T 
where 
    T: Float,
{   
    ema_bf(
        src.first().expect("first src not found"),
        bf.first_mut().expect("bf ema not found").unwrap_f(),
    )
}

#[allow(clippy::missing_panics_doc)]
pub fn ema_f<'a, T, V>(
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
    ema_bf(
        src.last().unwrap().borrow(), 
    &mut bf_ema(src, window, &true,)
    )
}

pub fn ema_f_abstr<T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>,
) -> T
where 
    T: Float,
    T: AddAssign,
    T: DivAssign,
{
    let window = args.first().expect("arg not found").unwrap_usize();
    ema_bf(
        src.first().expect("first src not found").last().unwrap(), 
        &mut bf_ema(src.first().expect("first src not found"), window, &true,)
    )
}

pub fn ema_coll<C, T>(
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
    let mut bf = bf_ema(&src[0..w], window, &false,);
    src
        [w..src.len()]
        .iter()
        .map(
            |v|
            ema_bf(
                v,
                &mut bf
            )
        )
        .collect()
}

pub fn ema_coll_abstr<C, T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>,
) -> C 
where
    T: Float,
    T: AddAssign,
    T: DivAssign,
    C: FromIterator<T>
{
    ema_coll(
        src.first().expect("first src not found"),
        args.first().expect("arg window not found in sma_coll_abstr").unwrap_usize()
    )
}