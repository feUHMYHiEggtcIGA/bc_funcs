use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::types::structures::*;

use crate::bf::rsi::bf_rsi;
use crate::ind::no_osc::trend::rma::rma_bf;


#[allow(clippy::missing_panics_doc)]
pub fn rsi<'a, T>(
    rma1: &T,
    rma2: &T,
) -> T
where 
    T: 'a,
    T: Float,
{
    let one_h = T::from(100.0).unwrap();
    one_h - (one_h / (T::one() + *rma1 / *rma2))
}

#[allow(clippy::implicit_hasher)]
pub fn rsi_bf<'a, T>(
    src: &T,
    bf: &mut FxHashMap<&'a str, T>,
    bf_rma1: &mut FxHashMap<&'a str, T>,
    bf_rma2: &mut FxHashMap<&'a str, T>,
) -> T
where 
    T: Float,
{
    let change = *src - bf["src"];
    let u = T::zero().max(change);
    let d = T::zero().max(-change);
    
    let rma1 = rma_bf(&u, bf_rma1);
    let rma2 = rma_bf(&d, bf_rma2);
    let res = rsi(&rma1, &rma2);
    bf.insert("src", *src);
    bf_rma1.insert("res", rma1);
    bf_rma2.insert("res", rma2);
    res
}

#[allow(clippy::pedantic)]
pub fn rsi_bf_abstr<T>(
    src: &SRC_ARG<T>,
    _: &ARGS<T>, 
    bf: & mut BF_VEC<T>
) -> T 
where 
    T: Float,
{
    let (bf1, rma) = bf.split_at_mut(1);
    let (rma1, rma2) = rma.split_at_mut(1);

    rsi_bf(
        src.first().unwrap(),
        bf1.last_mut().expect("bf rsi not found").unwrap_f(),
        rma1.last_mut().expect("bf rma1 in bf rsi not found").unwrap_f(),
        rma2.last_mut().expect("bf rma2 in bf rsi not found").unwrap_f(),
    )
}

#[allow(clippy::missing_panics_doc)]
pub fn rsi_f<T, V>(
    src: &SRC_ARG<V>,
    window: &usize,
) -> T
where 
    T: Float, 
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    V: Borrow<T>,
{
    let (
        mut bf,
        mut bf_rma1,
        mut bf_rma2,
    ) = bf_rsi(src, window, &true);
    rsi_bf(
        src.last().unwrap().borrow(),
        &mut bf, 
        &mut bf_rma1, 
        &mut bf_rma2
    )
}

pub fn rsi_f_abstr<T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>,
) -> T
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    let (
        mut bf,
        mut rma1,
        mut rma2
    ) = bf_rsi(src.first().unwrap(), args.first().expect("arg not found").unwrap_usize(), &true,);
    rsi_bf(
        src.last().unwrap().last().unwrap(), 
        &mut bf,
        &mut rma1,
        &mut rma2,
    )
}

pub fn rsi_coll<C, T>(
    src: &SRC_ARG<T>,
    window: &usize,
) -> C 
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    let w = window * 10 + 1;
    let (
        mut bf,
        mut rma1,
        mut rma2,
    ) = bf_rsi(&src[..w + 1], window, &false);
    src
        [w + 1..]
        .iter()
        .map(|v| rsi_bf(v, &mut bf, &mut rma1, &mut rma2))
        .collect()
}

pub fn rsi_coll_abstr<C, T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>,
) -> C 
where
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    rsi_coll(
        src.first().unwrap(),
        args.first().expect("arg window not found in rsi_coll_abstr").unwrap_usize()
    )
}