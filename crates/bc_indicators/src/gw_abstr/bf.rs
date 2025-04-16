use std::fmt::Debug;
use std::ops::{
    AddAssign,
    DivAssign,
};
use std::iter::Sum;

use num_traits::Float;
use bc_utils_lg::types::structures_abstr::*;
use bc_utils_lg::types::maps_abstr::*;
use bc_utils_lg::structs::settings::SETTINGS_IND;
use bc_utils_lg::traits::coll::AS_SLICE;
use bc_utils::other::roll_slice1;

use crate::gw_abstr::src::*;


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
#[must_use]
pub fn gw_func_bf_ind<'a, C, T>(
    src: &SRCS<T>,
    settings: &'static Vec<SETTINGS_IND>,
    map_bf_ind_abstr_: &'a MAP_FUNC_BF_IND<T>,
    map_args_ind_bf_: &MAP_ARGS<T>,
    exc_last: &bool,
    map_mod_coll: &MAP_MOD_COLL<C, T>,
    map2_args_mods_src: &MAP2_ARGS<T>,
) -> MAP_BF_VEC<'a, T>
where 
    T: Float,
    T: Sum,
    T: AddAssign,
    T: DivAssign,
    C: FromIterator<T>,
    C: IntoIterator<Item = T>,
    C: Clone,
    C: AS_SLICE<T>,
{   
    settings
        .iter()
        .map(
            |setting|
            {
                let key_uniq = setting.key_uniq.as_str();
                (
                    key_uniq,
                    map_bf_ind_abstr_
                    
                        .get(setting.key.as_str())
                        .expect("key rm not found")
                        (
                            gw_src_coll(
                                src,
                                &setting.used_src, 
                                map_mod_coll, 
                                &map2_args_mods_src[key_uniq],
                                Vec::new(),
                                |k, v| k.push(v),
                            )
                                .iter()
                                .map(C::as_slice)
                                .collect::<Vec<&[T]>>()
                                .as_slice(),
                            map_args_ind_bf_.get(key_uniq).unwrap(),
                            exc_last,
                    )
                )
            }
        )
        .collect()
}

pub fn gw_func_bf_mods<'a, C, T>(
    src: &SRCS<T>,
    settings: &'static Vec<SETTINGS_IND>,
    map_ind_coll_abstr_: &MAP_IND_COLL<C, T>,
    map_funcs_bf_mods_abstr: &'a MAP_FUNC_BF_MOD<T>,
    map_args_ind: &MAP_ARGS<T>,
    map1_args_mods: &MAP1_ARGS<T>,
    map2_args_mods_src: &MAP2_ARGS<T>,
    exc_last: &bool,
    map_mod_coll: &MAP_MOD_COLL<C, T>,
    init_map_coll: MAP_COLL<C>,
) -> MAP1_BF_VEC<'a, T>
where 
    T: Float,
    T: Sum,
    T: AddAssign,
    T: DivAssign,
    C: FromIterator<T>,
    C: IntoIterator<Item = T>,
    C: Clone,
    C: AS_SLICE<T>,
{
    let mut open = src["open"].clone();
    let mut high = src["high"].clone();
    let mut low = src["low"].clone();
    let mut close = src["close"].clone();
    settings
        .iter()
        .fold(
            (MAP1_BF_VEC::default(), init_map_coll),
            |(mut res, mut map_vec_ind), setting,| {
                let key_uniq = setting.key_uniq.as_str();
                let shift = &2;
                roll_slice1(open.as_mut_slice(), shift);
                roll_slice1(high.as_mut_slice(), shift);
                roll_slice1(low.as_mut_slice(), shift);
                roll_slice1(close.as_mut_slice(), shift);
                let src_rolled = SRC::from_iter([
                    ("open".to_string(), open.clone()),
                    ("high".to_string(), high.clone()),
                    ("low".to_string(), low.clone()),
                    ("close".to_string(), close.clone()),
                ]);
                let ind = map_ind_coll_abstr_[setting.key.as_str()](
                    gw_src_coll(
                        &src_rolled,
                        &setting.used_src, 
                        map_mod_coll, 
                        &map2_args_mods_src[key_uniq], 
                        vec![], 
                        |v1, v2| v1.push(v2),
                    )
                        .iter()
                        .map(|v| v.as_slice())
                        .collect::<Vec<&[T]>>()
                        .as_slice(),
                    &map_args_ind[key_uniq],
                );
                res.insert(
                    key_uniq,
                    setting
                        .used_mods
                        .iter()
                        .map(
                            |mod_| {
                                let key = mod_.key.as_str();
                                (
                                    key,
                                    map_funcs_bf_mods_abstr[key](
                                        ind.as_slice(),
                                        &mod_.used_indications
                                            .iter()
                                            .map(|ind_key_uniq| map_vec_ind[ind_key_uniq.as_str()].as_slice())
                                            .collect::<Vec<&[T]>>(),
                                        &map1_args_mods[key_uniq][key],
                                        exc_last,
                                    )
                                )
                                
                            }
                        )
                    .collect::<MAP_BF_VEC<T>>()
                );
                map_vec_ind.insert(key_uniq, ind);
                (res, map_vec_ind)
            }
        ).0
}