use std::borrow::Borrow;
use std::iter::Sum;

use bc_utils::other::coll1_roll_replace_el;
use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::structs::settings::SETTINGS_USED_SRC;
use bc_utils_lg::types::structures_abstr::*;
use bc_utils_lg::types::maps_abstr::*;
use rustc_hash::FxHasher;

use crate::gw_abstr::mods::*;


pub fn gw_src_f<T>(
    buff_src: &SRCS<T>,
    settings: &'static Vec<SETTINGS_USED_SRC>,
    map_mod_abstr_: &MAP_MOD_T<T>,
    map_map_args_mods: &MAP1_ARGS<T>,
    init_coll: Vec<T>,
    func_add: fn(&mut Vec<T>, T),
) -> Vec<T>
where 
    T: Float,
    T: Sum,
{
    settings
        .iter()
        .fold(
            (FxHashMap::default(), init_coll),
            |(mut map_src, mut res), setting,| {
                let src_new = {
                    let src_choised = &buff_src[setting.key.as_str()];
                    gw_mod_f(
                        &src_choised[src_choised.len() - 1 - setting.sub_from_last_i],
                        &map_src,
                        &setting.used_mods,
                        &map_map_args_mods[setting.key_uniq.as_str()],
                        map_mod_abstr_,
                    )
                };
                map_src.insert(
                    setting.key_uniq.as_str(),
                    src_new,
                );
                if setting.add_in_coll{
                    func_add(&mut res, src_new);
                }
                (map_src, res,)
            }
        )
        .1
}

pub fn gw_src_coll<T>(
    src: &SRCS<T>,
    settings: &'static Vec<SETTINGS_USED_SRC>,
    map_mod_coll_abstr_: &MAP_MOD_COLL<Vec<T>, T>,
    map_map_args_mods: &MAP1_ARGS<T>,
    init_coll: Vec<Vec<T>>,
    func_add: fn(&mut Vec<Vec<T>>, Vec<T>)
) -> Vec<Vec<T>>
where 
    T: Float,
    T: Sum,
{
    settings
        .iter()
        .fold(
            (FxHashMap::default(), init_coll),
            |(mut map_src, mut res), setting,| {
                let src_new: Vec<T> = {
                    gw_mod_coll(
                        coll1_roll_replace_el::<Vec<T>, _, _>(
                            src[&setting.key].clone().as_mut_slice(), 
                            &(setting.sub_from_last_i as i8),
                            T::nan()
                        )
                            .as_slice(),
                        &map_src,
                        &setting.used_mods,
                        &map_map_args_mods[setting.key_uniq.as_str()],
                        map_mod_coll_abstr_,
                    )
                };
                map_src.insert(
                    setting.key_uniq.as_str(),
                    src_new.clone(),
                );
                if setting.add_in_coll{
                    func_add(&mut res, src_new);
                }
                (map_src, res,)
            }
        )
        .1
}