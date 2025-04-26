use std::borrow::Borrow;

use num_traits::Float;
use bc_utils::nums;
use bc_utils_lg::types::structures::*;


pub fn profit_factor<'a, T, V>(
    pnl_qty: &SRC_ARG<V>,
) -> T
where 
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    V: Borrow<T>,
{
    let mut negative = T::zero();
    let mut positive = T::zero();
    let zero_ = &T::zero();

    for el in pnl_qty {
        let el_b = el.borrow();
        if el_b < zero_ {
            negative += *el_b
        } else if el_b > zero_ {
            positive += *el_b
        }
    }
    negative = negative.abs();
    if negative == *zero_ {
        positive / nums::dz(negative)
    } else {
        positive / negative
    }
}

pub fn profit_factor_abstr<'a, T, V>(
    src: &SRCS_ARG<T>,
) -> T
where 
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    V: Borrow<T>,
{
    profit_factor(&src[0])
}

pub fn profit_factor_coll<'a, C, T, V>(
    src: &SRCS_ARG<V>,
) -> C
where 
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    V: Borrow<T>,
    C: FromIterator<T>
{
    src
        .iter()
        .map(|v| profit_factor(v.borrow()))
        .collect()
}

pub fn profit_factor_coll_abstr<'a, C, T, V>(
    src: &SRCS1_ARG<V>,
) -> C
where 
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    V: Borrow<T>,
    C: FromIterator<T>
{
    profit_factor_coll::<C, _, _,>(&src[0])
}