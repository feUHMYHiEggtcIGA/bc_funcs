use std::iter::Sum;
use std::ops::{
    AddAssign,
    DivAssign
};
use std::vec;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::structs_and_types::maps_abstr::*;
use bc_utils_lg::structs_and_types::structures_abstr::*;
use bc_utils_lg::structs_and_types::settings::SETTINGS_IND;
use bc_utils_lg::traits::coll::{AS_ITER, AS_SLICE};

use crate::gw::{
    mods::*,
    src::*,
};


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::implicit_hasher)]
#[allow(clippy::ptr_arg)]
#[allow(clippy::too_many_arguments)]
pub fn gw_ind_bf<T>(
    buff_src: &SRCS<T>,
    settings: &'static Vec<SETTINGS_IND>,
    map_ind_bf_: &MAP_IND_T_BF<T, T>,
    map_mods_bf_all_: &MAP_MOD_T_BF<T, T>,
    map_mod_f_: &MAP_MOD_T<T, T>,
    map_args_ind_bf_: &MAP_ARGS<T, T>,
    map1_args_mods_ind_: &MAP1_ARGS<T, T>,
    map2_args_mods_src_: &MAP2_ARGS<T, T>,
    map_bf_ind: &mut MAP_BF_VEC<T>,
    map_bf_mods: &mut MAP1_BF_VEC<T>,
) -> FxHashMap<&'static str, T>
where 
    T: Float,
    T: Sum,
    T: AddAssign,
    T: DivAssign,
{ 
    settings
        .iter()
        .fold(
            FxHashMap::default(),
            |mut map, setting, | {
                let key_uniq_str = setting.key_uniq.as_str();
                map.insert(
                    key_uniq_str,
                    gw_mod_bf(
                        &map_ind_bf_[setting.key.as_str()](
                            &gw_src_f(
                                buff_src,
                                &setting.used_src, 
                                map_mod_f_, 
                                &map2_args_mods_src_[setting.key_uniq.as_str()],
                                vec![],
                                |v1, v2| v1.push(v2),
                            ),
                            &map_args_ind_bf_[key_uniq_str],
                            map_bf_ind.get_mut(key_uniq_str).expect("bf not found"),
                        ), 
                        &map,
                        &setting.used_mods,
                        map1_args_mods_ind_.get(key_uniq_str).expect("args mods not found"), 
                        map_mods_bf_all_, 
                        map_bf_mods.get_mut(key_uniq_str).expect("bf mods not found"),
                    )
                );
                map
            }
        )
}

pub fn gw_ind_coll<C, M, T>(
    src: &SRCS<T>,
    settings: &'static Vec<SETTINGS_IND>,
    map_ind_coll_abstr_: &MAP_IND_COLL<C, T, T>,
    map_args_: &MAP_ARGS<T, T>,
    map_mod_coll_: &MAP_MOD_COLL<C, T, T>,
    map_map_map_args_mods_src_: &MAP2_ARGS<T, T>,
    init_coll: M,
    func_add: fn(&mut M, C),
) -> FxHashMap<&'static str, C>
where 
    T: Float,
    T: Sum,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    C: FromIterator<T>,
    C: IntoIterator<Item = T>,
    C: Clone,
    C: AS_SLICE<T>,
    M: Clone,
    M: AS_ITER<C>,
{
    settings
        .iter()
        .map(
            |setting| {
                let key_uniq = setting.key_uniq.as_str();
                (
                    key_uniq,
                    map_ind_coll_abstr_[setting.key.as_str()](
                        gw_src_coll(
                            src, 
                            &setting.used_src, 
                            map_mod_coll_, 
                            &map_map_map_args_mods_src_[key_uniq],
                            init_coll.clone(),
                            func_add,
                        )
                            .iter()
                            .map(C::as_slice)
                            .collect::<Vec<&[T]>>()
                            .as_slice(),
                        &map_args_[key_uniq],
                    )
                )
            }
        )
        .collect()
}