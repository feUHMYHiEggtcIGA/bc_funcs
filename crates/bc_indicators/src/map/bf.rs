use std::ops::{
    AddAssign, 
    DivAssign,
};

use num_traits::Float;
use bc_utils_lg::types::funcs::*;
use bc_utils_lg::types::maps::*;
use bc_utils_lg::statics::funcs::fn_ind_bf_abstr_default;

use crate::bf::trend_ma::bf_trend_ma_abstr;
use crate::bf::{
    ema::bf_ema_abstr,
    sma::bf_sma_abstr,
    rma::bf_rma_abstr,
    rsi::bf_rsi_abstr,
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
        ("nohesi", bf_nohesi_abstr as FUNC_BF_IND<T>),
        ("avg", fn_ind_bf_abstr_default as FUNC_BF_IND<T>),
        ("trend_ma", bf_trend_ma_abstr as FUNC_BF_IND<T>),
    ])
}
