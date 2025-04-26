use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::types::structures::{ARGS, BF_VEC, SRCS_ARG, SRC_ARG};

use crate::bf::nohesi::bf_nohesi;


pub fn nohesi_bf<T>(
    v: &T,
    hesi: &T,
    bf: &mut FxHashMap<&'static str, T>
) -> T
where 
    T: Float,
{
    let v = *v;
    let hesit = v * *hesi;
    let peak_bf = bf["peak"];
    let btm_bf = bf["btm"];
    let peak;
    let btm ;
    
    if v > peak_bf {
        peak = v;
        btm = v - hesit;
    } else if v < btm_bf {
        peak = v + hesit;
        btm = v;
    } else {
        peak = peak_bf;
        btm = btm_bf;
    }
    bf.insert("peak", peak);
    bf.insert("btm", btm);
    if btm < btm_bf || peak > peak_bf {
        bf.insert("res", v);
        return v;
    }
    bf["res"]
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
#[allow(clippy::pedantic)]
pub fn nohesi_bf_abstr<T>(
    v: &T,
    _: &SRC_ARG<T>,
    args: &ARGS<T>, 
    bf: &mut BF_VEC<T>,
) -> T 
where 
    T: Float,
{
    nohesi_bf(
        v,
        args
            .first()
            .unwrap()
            .unwrap_f(),
        bf
            .first_mut()
            .unwrap()
            .unwrap_f()
    )
}

pub fn nohesi_f<T>(
    v: &SRC_ARG<T>,
    hesi: &T,
) -> T
where 
    T: Float,
{
    nohesi_bf(v.last().unwrap(), hesi, &mut bf_nohesi(v, hesi, &true))
}

pub fn nohesi_f_abstr<T>(
    src: &SRC_ARG<T>,
    _: &SRCS_ARG<T>,
    args: &ARGS<T>,
) -> T
where 
    T: Float,
{
    nohesi_f(src, args[0].unwrap_f())
}

pub fn nohesi_coll<C, T>(
    src: &SRC_ARG<T>,
    hesi: &T,
) -> C
where
    T: Float,
    C: FromIterator<T>,
{
    let mut bf = bf_nohesi(&src[..3], hesi, &true);
    src
        [2..]
        .iter()
        .map(|v| nohesi_bf(v.borrow(), hesi, &mut bf))
        .collect::<C>()
}

pub fn nohesi_coll_abstr<C, T>(
    src: &SRC_ARG<T>,
    _: &SRCS_ARG<T>,
    args: &ARGS<T>,
) -> C
where
    T: Float,
    C: FromIterator<T>,
{
    nohesi_coll::<C, T>(src, args[0].unwrap_f())
}
