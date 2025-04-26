use std::iter::Sum;

use num_traits::Float;
use bc_utils_lg::types::maps::*;
use bc_utils_lg::types::funcs::*;

use crate::ind::{
    no_osc::trend::ema::{ema_bf_abstr, ema_f_abstr, ema_coll_abstr},
    no_osc::trend::sma::{sma_bf_abstr, sma_f_abstr, sma_coll_abstr},
    no_osc::trend::rma::{rma_bf_abstr, rma_f_abstr, rma_coll_abstr},
    osc::other::rsi::{rsi_bf_abstr, rsi_f_abstr, rsi_coll_abstr},
    osc::trend::tqo_b::{tqo_b_bf_abstr, tqo_b_f_abstr, tqo_b_coll_abstr},
};


#[must_use]
pub fn map_ind_t_bf<T>() -> MAP_IND_T_BF<T>
where 
    T: Float,
    T: Sum,
{
    MAP_IND_T_BF::from_iter([
        ("sma", sma_bf_abstr as IND_T_BF<T>),
        ("ema", ema_bf_abstr as IND_T_BF<T>),
        ("bfa", rma_bf_abstr as IND_T_BF<T>),
        ("rsi", rsi_bf_abstr as IND_T_BF<T>),
        ("tqo_b", tqo_b_bf_abstr as IND_T_BF<T>),
    ])
}

#[must_use]
pub fn map_ind_t<T>() -> MAP_IND_T<T> 
where 
    T: Float,
    T: Sum,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    MAP_IND_T::from_iter([
        ("sma", sma_f_abstr as IND_T<T>),
        ("ema", ema_f_abstr as IND_T<T>),
        ("bfa", rma_f_abstr as IND_T<T>),
        ("rsi", rsi_f_abstr as IND_T<T>),
        ("tqo_b", tqo_b_f_abstr as IND_T<T>),
    ])
}

#[must_use]
pub fn map_ind_coll<C, T>() -> MAP_IND_COLL<C, T>
where 
    T: Float,
    T: Sum,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    MAP_IND_COLL::from_iter([
        ("sma", sma_coll_abstr as IND_COLL<C, T>),
        ("ema", ema_coll_abstr as IND_COLL<C, T>),
        ("bfa", rma_coll_abstr as IND_COLL<C, T>),
        ("rsi", rsi_coll_abstr as IND_COLL<C, T>),
        ("tqo_b", tqo_b_coll_abstr as IND_COLL<C, T>),
    ])
}