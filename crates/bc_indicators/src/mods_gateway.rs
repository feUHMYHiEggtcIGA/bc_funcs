use std::iter::Sum;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::enums::indicators::*;
use bc_utils_lg::structs::settings::*;
// use bc_utils_lg::structs::src::*;
use bc_utils_lg::types::indicators::*;

use crate::mods::*;


#[must_use]
pub fn map_map_args_mod<T>(
    settings: &'static Vec<SETTINGS_IND>,
) -> MAP_aMAP_ARGS<T>
where 
    T: Float,
{
    settings
        .iter()
        .map(
            |setting|
            (
                setting.key_uniq.as_str(),
                setting.used_mods
                    .iter()
                    .map(
                        |mod_| 
                        {
                            let key = mod_.key.as_str();
                            (
                                key,
                                {
                                    match key {
                                        "avg" => vec![T_ARGS::None(())],
                                        "nohesi" => vec![T_ARGS::Float(T::from(mod_.kwargs_f64["hesi"]).expect("invalid cast"))],
                                        _ => panic!("mod not found"),
                                    }
                                }
                            )
                        }
                    )
                    .collect()
            )
        )
        .collect()
}

pub fn map_mod_all_abstr<T>() -> MAP_aMOD_T_RM_ABSTR<T>
where 
    T: Float,
    T: std::ops::AddAssign,
{
    FxHashMap::from_iter([
        ("nohesi", nohesi_rm_abstr as MOD_T_RM_aABSTR<T>,),
        ("avg", avg_rm_abstr as MOD_T_RM_aABSTR<T>,),
    ])
}

pub fn map_mod_rm_abstr<T>() -> MAP_aMOD_T_RM_ABSTR<T>
where 
    T: Float,
    T: std::ops::AddAssign,
{
    FxHashMap::from_iter([
        ("nohesi", nohesi_rm_abstr as MOD_T_RM_aABSTR<T>,),
    ])
}

pub fn map_mod_abstr<T>() -> MAP_aMOD_T_ABSTR<T>
where 
    T: Float,
    T: std::ops::AddAssign,
{
    FxHashMap::from_iter([
        ("avg", avg_abstr as MOD_T_aABSTR<T>,),
    ])
}


pub fn gw_mod_t_rm_abstr<T>(
    ind: &T,
    map_ind: &FxHashMap<&'static str, T>,
    settings: &Vec<SETTINGS_USED_MODS>,
    map_args_: &MAP_aARGS<T>,
    map_mod_t_rm_abstr_: &MAP_aMOD_T_RM_ABSTR<T>,
    rm: &mut MAP_aRM_VEC<T>,
) -> T
where
    T: Float,
    T: Sum,
{
    settings
        .iter()
        .fold(
            *ind,
            |ind_iter, setting| {
                let key = setting.key.as_str();
                map_mod_t_rm_abstr_.get(key).unwrap()(
                    &ind_iter,
                    &setting.used_indications
                        .iter()
                        .map(|usd_ind| map_ind.get(usd_ind.as_str()).expect("ind no found"))
                        .collect::<Vec<&T>>(),
                    map_args_.get(key).unwrap(),
                    rm.get_mut(key).unwrap(),
                )
            }
        )
}