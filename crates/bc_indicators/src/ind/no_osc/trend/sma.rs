use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::structs_and_types::structures_abstr::*;
use bc_utils::other::coll1_roll_replace_el;

use crate::bf::sma::bf_sma;


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::implicit_hasher)]
pub fn sma_bf<T>(
    src: T,
    window: &usize,
    buff: &mut FxHashMap<&str, Vec<T>>,
) -> T
where
    T: Float,
    T: std::iter::Sum,
{
    let v = coll1_roll_replace_el::<Vec<T>, _, _,>(
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
pub fn sma_bf_abstr<T>(
    src: &SRC_ARG<T>,
    args: &ARGS<T, T>, 
    bf: &mut BF_VEC<T>
) -> T
where 
    T: Float,
    T: std::iter::Sum,
{
    sma_bf(
        *src.first().expect("first src not found"),
        args.first().expect("first arg not found").unwrap_usize(),
        bf.first_mut().expect("bf sma not found").unwrap_vec_f(),
    )
}

pub fn sma_f<T>(
    src: &SRC_ARG<T>,
    window: &usize,
) -> T 
where 
    T: Float,
    T: std::iter::Sum,
{
    sma_bf(*src.last().unwrap(), window, &mut bf_sma(src, window, &true))
}

pub fn sma_f_abstr<T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T, T>,
) -> T
where 
    T: Float,
    T: std::iter::Sum,
{
    sma_f(src.first().expect("first src not found"), args.first().unwrap().unwrap_usize())
}

pub fn sma_coll<C, T>(
    src: &SRC_ARG<T>,
    window: &usize,
) -> C 
where 
    T: Float,
    T: std::iter::Sum,
    C: FromIterator<T>
{
    let mut bf= bf_sma(
        &src[..*window + 1], 
        window, 
        &true,
    );
    src
        [*window..]
        .iter()
        .map(
            |v|
            sma_bf(
                *v,
                window,
                &mut bf
            )
        )
        .collect()
}

pub fn sma_coll_abstr<C, T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T, T>,
) -> C 
where
    T: Float,
    T: std::iter::Sum,
    C: FromIterator<T>
{
    sma_coll(
        src.first().expect("first src not found"),
        args.first().expect("arg window not found in sma_coll_abstr").unwrap_usize()
    )
}
