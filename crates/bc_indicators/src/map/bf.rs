use std::ops::{
    AddAssign, 
    DivAssign,
};

use num_traits::Float;
use bc_utils_lg::types::funcs_abstr::*;
use bc_utils_lg::types::maps_abstr::*;
use bc_utils_lg::statics::funcs::fn_bf_mod_abstr_default;

use crate::bf::{
    ema::bf_ema_abstr,
    sma::bf_sma_abstr,
    rma::bf_rma_abstr,
    rsi::bf_rsi_abstr,
    tqo_b::bf_tqo_b_abstr,
    nohesi::bf_nohesi_abstr,
};


pub fn map_func_bf_ind<'a, T>() -> MAP_FUNC_BF_IND<'a, T>
where 
    T: Float,
    T: AddAssign,
    T: DivAssign,
{
    MAP_FUNC_BF_IND::from_iter([
        ("sma", bf_sma_abstr as FUNC_BF_IND<T>),
        ("ema", bf_ema_abstr as FUNC_BF_IND<T>),
        ("rma", bf_rma_abstr as FUNC_BF_IND<T>),
        ("rsi", bf_rsi_abstr as FUNC_BF_IND<T>),
        ("tqo_b", bf_tqo_b_abstr as FUNC_BF_IND<T>),
    ])
}

pub fn map_func_bf_mod<'a, T>() -> MAP_FUNC_BF_MOD<'a, T>
where 
    T: Float,
    T: AddAssign,
    T: DivAssign,
{
    MAP_FUNC_BF_MOD::from_iter([
        ("nohesi", bf_nohesi_abstr as FUNC_BF_MOD<T>),
        ("avg", fn_bf_mod_abstr_default as FUNC_BF_MOD<T>),
    ])
}
