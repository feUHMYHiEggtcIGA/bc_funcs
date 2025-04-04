/// all rm functions are dirty because they change rm values.
/// this is done for ease of use, sinceit is not particularly
/// convenient to implement the return of
/// functions, although this is done explicitly.

use std::ops;
use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::enums::indicators::*;
use bc_utils_lg::structs::src::*;
use bc_utils_lg::types::indicators::*;

use bc_utils::transf;

use crate::rm::{self, rm_sma};


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::implicit_hasher)]
pub fn sma_rm<T>(
    src: T,
    window: &usize,
    buff: &mut FxHashMap<&str, Vec<T>>,
) -> T
where
    T: Float,
    T: std::iter::Sum,
{
    let v = transf::coll1_roll_replace_el::<_, _, Vec<T>>(
        buff
            .remove("src")
            .unwrap()
            .as_mut_slice(), 
        &-1,
        src,
    );
    buff.insert("src", v);
    buff["src"].iter().map(|x| *x.borrow()).sum::<T>() 
        / T::from(*window).unwrap()
}

#[allow(clippy::ptr_arg)]
pub fn sma_rm_abstr<T>(
    src: &SRC_EL<T>, 
    args: &Vec<T_ARGS<T>>, 
    rm: &mut Vec<T_HASHMAP<T>>
) -> T
where 
    T: Float,
    T: std::iter::Sum,
{
    sma_rm(
        src.open,
        args.first().expect("first arg not found").unwrap_usize(),
        rm.first_mut().expect("rm sma not found").unwrap_vec_f(),
    )
}

pub fn sma_f<T>(
    src: &[T],
    window: &usize,
) -> T 
where 
    T: Float,
    T: std::iter::Sum,
{
    sma_rm(*src.last().unwrap(), window, &mut rm_sma(src, window, &true))
}

pub fn sma_f_abstr<T>(
    src: &SRC<T>,
    args: &Vec<T_ARGS<T>>,
) -> T
where 
    T: Float,
    T: std::iter::Sum,
{
    sma_f(src.open, args.first().unwrap().unwrap_usize())
}

pub fn sma_coll<C, T>(
    src: &[T],
    window: &usize,
) -> C 
where 
    T: Float,
    T: std::iter::Sum,
    C: FromIterator<T>
{
    let mut rm= rm_sma(&src[..*window + 1], window, &true,);
    src
        [*window..]
        .iter()
        .map(
            |v|
            sma_rm(
                *v,
                window,
                &mut rm
            )
        )
        .collect()
}

pub fn sma_coll_abstr<C, T>(
    src: &SRC<T>,
    args: &ARGS<T>,
) -> C 
where
    T: Float,
    T: std::iter::Sum,
    C: FromIterator<T>
{
    sma_coll(
        src.open,
        args.first().expect("arg window not found in sma_coll_abstr").unwrap_usize()
    )
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

#[allow(clippy::missing_panics_doc)]
pub fn alpha_ema<T>(
    window: &T,
) -> T 
where
    T: Float,
{
    T::from(2.0).unwrap() / (*window + T::one())
}

#[allow(clippy::implicit_hasher)]
pub fn ema_rm<T>(
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

#[allow(clippy::missing_panics_doc)]
pub fn ema_f<'a, T, V>(
    src: &[V],
    window: &usize,
) -> T
where 
    T: Float,
    T: 'a,
    T: ops::AddAssign,
    T: ops::DivAssign,
    V: Borrow<T>,
{
    ema_rm(
        src.last().unwrap().borrow(), 
    &mut rm::rm_ema(src, window, &true,)
    )
}

#[allow(clippy::needless_borrows_for_generic_args)]
pub fn ema_rm_abstr<T>(
    src: &SRC_EL<T>, 
    _: &Vec<T_ARGS<T>>, 
    rm: &mut Vec<T_HASHMAP<T>>
) -> T 
where 
    T: Float,
{   
    ema_rm(
        &src.open,
        rm.first_mut().expect("rm ema not found").unwrap_f(),
    )
}

pub fn ema_f_abstr<T>(
    src: &SRC<T>,
    args: &Vec<T_ARGS<T>>,
) -> T
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    let window = args.first().expect("arg not found").unwrap_usize();
    ema_rm(
        src.open.last().expect("open last price not found"), 
        &mut rm::rm_ema(src.open, window, &true,)
    )
}

pub fn ema_coll<C, T>(
    src: &[T],
    window: &usize,
) -> C 
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    let w = *window * 10;
    let mut rm = rm::rm_ema(&src[0..w + 1], window, &true,);
    src
        [w..src.len()]
        .iter()
        .map(
            |v|
            ema_rm(
                v,
                &mut rm
            )
        )
        .collect()
}

pub fn ema_coll_abstr<C, T>(
    src: &SRC<T>,
    args: &ARGS<T>,
) -> C 
where
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    ema_coll(
        src.open,
        args.first().expect("arg window not found in sma_coll_abstr").unwrap_usize()
    )
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


pub fn alpha_rma<T: Float>(
    window: &T,
) -> T {
    T::one() / *window
}

#[allow(clippy::implicit_hasher)]
pub fn rma_rm<T: Float>(
    src: &T,
    buff: &mut FxHashMap<&str, T>
) -> T {
    let res = rma(src, &buff["res"], &buff["alpha"]);
    buff.insert("res", res);
    res
}

#[allow(clippy::missing_panics_doc)]
pub fn rma_f<'a, T, V>(
    src: &[V],
    window: &usize,
) -> T
where 
    T: Float,
    T: 'a,
    T: ops::AddAssign,
    T: ops::DivAssign,
    V: Borrow<T>,
{
    rma_rm(src.last().unwrap().borrow(), &mut rm::rm_rma(src, window, &true,))
}

#[allow(clippy::needless_borrows_for_generic_args)]
pub fn rma_rm_abstr<T>(
    src: &SRC_EL<T>, 
    _: &Vec<T_ARGS<T>>, 
    rm: & mut Vec<T_HASHMAP<T>>
) -> T 
where 
    T: Float,
{
    rma_rm(
        &src.open,
        rm.first_mut().expect("rm rma not found").unwrap_f(),
    )
}

pub fn rma_f_abstr<T>(
    src: &SRC<T>,
    args: &Vec<T_ARGS<T>>,
) -> T
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    let window = args.first().expect("arg not found").unwrap_usize();
    rma_rm(
        src.open.last().expect("open last price not found"), 
        &mut rm::rm_rma(src.open, window, &true,)
    )
}

pub fn rma_coll<C, T>(
    src: &[T],
    window: &usize,
) -> C 
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    let w = *window * 10;
    let mut rm = rm::rm_rma(&src[0..=w + 1], window, &true,);
    src
        [w..src.len()]
        .iter()
        .map(
            |v|
            rma_rm(
                v,
                &mut rm
            )
        )
        .collect()
}

pub fn rma_coll_abstr<C, T>(
    src: &SRC<T>,
    args: &ARGS<T>,
) -> C 
where
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    rma_coll(
        src.open,
        args.first().expect("arg window not found in sma_coll_abstr").unwrap_usize()
    )
}

#[allow(clippy::missing_panics_doc)]
pub fn rational_quadratic_f<'a,T>(
    src: &[&'a T],
    window: &usize,
    relative_weight: &T,
    start_at_bar: &usize,
) -> T 
where 
    T: Float,
    T: 'a,
    T: ops::AddAssign,
    T: ops::MulAssign,
    T: ops::DivAssign,
{
    let mut current_weight: T = T::zero();
    let mut cumulative_weight: T = T::zero();
    let two = T::from(2.0).unwrap();
    let one = T::one();
    let window_t = T::from(*window).unwrap();


    for (i, y) in src
        .iter()
        .skip(src.len() - *window)
        .enumerate()
    {
        let w: T = (
            (one + T::from(*start_at_bar - i).unwrap().powf(two)) 
            / ((window_t).powf(two) * two * *relative_weight))
                .powf(-(*relative_weight)
        );
        current_weight += w * **y;
        cumulative_weight += w;
    }
    transf::nz(current_weight / cumulative_weight, T::zero())
}
