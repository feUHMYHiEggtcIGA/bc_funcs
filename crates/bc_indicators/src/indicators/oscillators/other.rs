use std::borrow::Borrow;

use bc_utils_lg::types::indicators::{MAP_aARGS, ARGS};
use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::enums::indicators::*;
use bc_utils_lg::structs::src::*;

use crate::indicators::no_oscillators::trend;
use crate::rm::{self, rm_rsi};

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
pub fn rsi_rm<'a, T>(
    src: &T,
    rm: &mut FxHashMap<&'a str, T>,
    rm_rma1: &mut FxHashMap<&'a str, T>,
    rm_rma2: &mut FxHashMap<&'a str, T>,
) -> T
where 
    T: Float,
{
    let change = *src - rm["src"];
    let u = T::zero().max(change);
    let d = T::zero().max(-change);
    
    let rma1 = trend::rma_rm(&u, rm_rma1);
    let rma2 = trend::rma_rm(&d, rm_rma2);
    let res = rsi(&rma1, &rma2);
    rm.insert("src", *src);
    rm_rma1.insert("res", rma1);
    rm_rma2.insert("res", rma2);
    res
}

#[allow(clippy::pedantic)]
pub fn rsi_rm_abstr<T>(
    src: &SRC_EL<T>,
    _: &Vec<T_ARGS<T>>, 
    rm: & mut Vec<T_HASHMAP<T>>
) -> T 
where 
    T: Float,
{
    let (rm1, rma) = rm.split_at_mut(1);
    let (rma1, rma2) = rma.split_at_mut(1);

    rsi_rm(
        &src.open,
        rm1.last_mut().expect("rm rsi not found").unwrap_f(),
        rma1.last_mut().expect("rm rma1 in rm rsi not found").unwrap_f(),
        rma2.last_mut().expect("rm rma2 in rm rsi not found").unwrap_f(),
    )
}

#[allow(clippy::missing_panics_doc)]
pub fn rsi_f<T, V>(
    src: &[V],
    window: &usize,
) -> T
where 
    T: Float, 
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    V: Borrow<T>,
{
    let (
        mut rm,
        mut rm_rma1,
        mut rm_rma2,
    ) = rm::rm_rsi(src, window, &true,);
    rsi_rm(
        src.last().unwrap().borrow(), 
        &mut rm, 
        &mut rm_rma1, 
        &mut rm_rma2
    )
}

pub fn rsi_f_abstr<T>(
    src: &SRC<T>,
    args: &Vec<T_ARGS<T>>,
) -> T
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    let (
        mut rm,
        mut rma1,
        mut rma2
    ) = rm::rm_rsi(src.open, args.first().expect("arg not found").unwrap_usize(), &true,);
    rsi_rm(
        src.open.last().expect("open last price not found"), 
        &mut rm,
        &mut rma1,
        &mut rma2,
    )
}

pub fn rsi_coll<C, T>(
    src: &[T],
    window: &usize,
) -> C 
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    let w = *window * 10 + 1;
    let (
        mut rm,
        mut rma1,
        mut rma2,
    ) = rm_rsi(
        &src[..w + 1], 
        window,
        &true,
    );
    src
        [w..src.len()]
        .iter()
        .map(
            |v|
            rsi_rm(
                v, 
                &mut rm,
                &mut rma1,
                &mut rma2,
            )
        )
        .collect()
}

pub fn rsi_coll_abstr<C, T>(
    src: &SRC<T>,
    args: &ARGS<T>,
) -> C 
where
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    rsi_coll(
        src.open,
        args.first().expect("arg window not found in rsi_coll_abstr").unwrap_usize()
    )
}