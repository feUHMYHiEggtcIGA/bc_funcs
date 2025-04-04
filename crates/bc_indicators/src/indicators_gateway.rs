use std::iter::Sum;

// use bc_utils_lg::structs::settings::SETTINGS_USED_MODS;
use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::structs::src::{
    SRC_EL as SRC_EL_SRCT,
    SRC as SRC_SRCT,
};
use bc_utils_lg::structs::settings::SETTINGS_IND;
#[allow(clippy::wildcard_imports)]
use bc_utils_lg::enums::indicators::*;
#[allow(clippy::wildcard_imports)]
use bc_utils_lg::types::indicators::*;

#[allow(clippy::wildcard_imports)]
use crate::indicators::no_oscillators::trend::*;
#[allow(clippy::wildcard_imports)]
use crate::indicators::oscillators::trend::*;
#[allow(clippy::wildcard_imports)]
use crate::indicators::oscillators::other::*;
use crate::mods_gateway::*;


#[must_use]
pub fn map_ind_t_rm_abstr<T>() -> MAP_aIND_T_RM_ABSTR<T>
where 
    T: Float,
    T: Sum,
{
    MAP_aIND_T_RM_ABSTR::from_iter([
        ("sma", sma_rm_abstr as IND_T_RM_aABSTR<T>),
        ("ema", ema_rm_abstr as IND_T_RM_aABSTR<T>),
        ("rma", rma_rm_abstr as IND_T_RM_aABSTR<T>),
        ("rsi", rsi_rm_abstr as IND_T_RM_aABSTR<T>),
        ("tqo_b", tqo_b_rm_abstr as IND_T_RM_aABSTR<T>),
    ])
}

#[must_use]
pub fn map_ind_t_abstr<T>() -> MAP_aIND_T_ABSTR<T> 
where 
    T: Float,
    T: Sum,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    MAP_aIND_T_ABSTR::from_iter([
        ("sma", sma_f_abstr as IND_T_aABSTR<T>),
        ("ema", ema_f_abstr as IND_T_aABSTR<T>),
        ("rma", rma_f_abstr as IND_T_aABSTR<T>),
        ("rsi", rsi_f_abstr as IND_T_aABSTR<T>),
        ("tqo_b", tqo_b_f_abstr as IND_T_aABSTR<T>),
    ])
}

#[must_use]
pub fn map_ind_coll_abstr<C, T>() -> MAP_aIND_COLL_ABSTR<C, T>
where 
    T: Float,
    T: Sum,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    MAP_aIND_COLL_ABSTR::from_iter([
        ("sma", sma_coll_abstr as IND_COLL_aABSTR<C, T>),
        ("ema", ema_coll_abstr as IND_COLL_aABSTR<C, T>),
        ("rma", rma_coll_abstr as IND_COLL_aABSTR<C, T>),
        ("rsi", rsi_coll_abstr as IND_COLL_aABSTR<C, T>),
        ("tqo_b", tqo_b_coll_abstr as IND_COLL_aABSTR<C, T>),
    ])
}

// add nohesi mod
pub fn map_func_window_func_rm_abstr<T>() -> MAP_aFUNC_USIZE_ABSTR<T> 
where 
    T: Float,
{
    MAP_aFUNC_USIZE_ABSTR::from_iter([
        ("sma", |a: &ARGS<T>| -> usize {*a.first().unwrap().unwrap_usize() + 1} as FUNC_aUSIZE_aABSTR<T>),
        ("ema", |a: &ARGS<T>| -> usize {a.first().unwrap().unwrap_usize() * 10 + 1} as FUNC_aUSIZE_aABSTR<T>),
        ("rma", |a: &ARGS<T>| -> usize {a.first().unwrap().unwrap_usize() * 10 + 1} as FUNC_aUSIZE_aABSTR<T>),
        ("rsi", |a: &ARGS<T>| -> usize {a.first().unwrap().unwrap_usize() * 10 + 2} as FUNC_aUSIZE_aABSTR<T>),
        ("tqo_b", window_tqo_b_abstr as FUNC_aUSIZE_aABSTR<T>),
        ("nohesi", |_: &ARGS<T>| -> usize {3} as FUNC_aUSIZE_aABSTR<T>),
    ])
}

pub fn gw_map_func_window_ind_abstr<T>(
    settings: &'static Vec<SETTINGS_IND>,
    map_func_window_ind_abstr: &MAP_aFUNC_USIZE_ABSTR<T>,
    map_args_: &MAP_aARGS<T>,
) -> MAP_aUSIZE 
where 
    T: Float,
{
    settings
        .iter()
        .map(
            |setting| {
                let key_uniq = setting.key_uniq.as_str();
                (
                    key_uniq,
                    map_func_window_ind_abstr[setting.key.as_str()](&map_args_[key_uniq])
                )
            }
        )
        .collect()
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
#[must_use]
pub fn map_args_ind<T>(
    settings: &'static Vec<SETTINGS_IND>
) -> MAP_aARGS<T>
where  
    T: Float,
{
    settings
        .iter()
        .map(
            |setting| 
            (
                setting.key_uniq.as_str(),
                match setting.key.as_str() {
                    "sma" | "rma" | "ema" | "rsi" => vec![T_ARGS::<T>::Usize(setting.kwargs_usize["window"])],
                    "tqo_b" => vec![
                        T_ARGS::<T>::Usize(setting.kwargs_usize["window_ema_fast"]),
                        T_ARGS::<T>::Usize(setting.kwargs_usize["window_ema_slow"]),
                        T_ARGS::<T>::Usize(setting.kwargs_usize["window_trend"]),
                        T_ARGS::<T>::Usize(setting.kwargs_usize["window_noise"]),
                        T_ARGS::<T>::Usize(setting.kwargs_usize["add_iters"]),
                        T_ARGS::<T>::Float(T::from(setting.kwargs_f64["correlation_factor"]).unwrap()),
                        T_ARGS::<T>::String(setting.kwargs_string["noise_type"].clone()),
                    ],
                    _ => panic!("key indication unknown"),
                }
            )
        )
        .collect()
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::implicit_hasher)]
#[allow(clippy::ptr_arg)]
#[allow(clippy::too_many_arguments)]
pub fn gw_ind_rm_abstr<T>(
    src: &SRC_EL_SRCT<T>,
    settings: &'static Vec<SETTINGS_IND>,
    map_indicators_rm_: &MAP_aIND_T_RM_ABSTR<T>,
    map_mods_all_abstr_: &MAP_aMOD_T_RM_ABSTR<T>,
    map_args_ind_rm_abstr_: &MAP_aARGS<T>,
    map_args_mods_all_abstr_: &MAP_aMAP_ARGS<T>,
    map_rm_ind: &mut MAP_aRM_VEC<T>,
    map_rm_mods: &mut MAP_aMAP_RM_VEC<T>,
) -> FxHashMap<&'static str, T>
where 
    T: Float,
    T: Sum,
{ 
    settings
        .iter()
        .fold(
            FxHashMap::default(),
            |mut map, setting, | {
                let key_uniq_str = setting.key_uniq.as_str();
                map.insert(
                    key_uniq_str,
                    gw_mod_t_rm_abstr(
                        &map_indicators_rm_[setting.key.as_str()](
                            // todo: add support choice src from a key
                            src,
                            &map_args_ind_rm_abstr_[key_uniq_str],
                            map_rm_ind.get_mut(key_uniq_str).expect("rm not found"),
                        ), 
                        &map,
                        &setting.used_mods,
                        map_args_mods_all_abstr_.get(key_uniq_str).expect("args mods not found"), 
                        map_mods_all_abstr_, 
                        map_rm_mods.get_mut(key_uniq_str).expect("rm mods not found"),
                    )
                );
                map
            }
        )
}

pub fn gw_ind_coll_abstr<C, T>(
    src: &SRC_SRCT<T>,
    settings: &'static Vec<SETTINGS_IND>,
    map_ind_coll_abstr_: &MAP_aIND_COLL_ABSTR<C, T>,
    map_args_: &MAP_aARGS<T>,
) -> FxHashMap<&'static str, C>
where 
    T: Float,
    T: Sum,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>
{
    settings
        .iter()
        .map(
            |setting| {
                let key_uniq = setting.key_uniq.as_str();
                (
                    key_uniq,
                    map_ind_coll_abstr_[setting.key.as_str()](src, &map_args_[key_uniq])
                )
            }
        )
        .collect()
}