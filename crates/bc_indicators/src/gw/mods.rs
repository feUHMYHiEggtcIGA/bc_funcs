use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::structs_and_types::settings::SETTINGS_USED_MODS;
use bc_utils_lg::structs_and_types::maps_abstr::*;
use bc_utils_lg::traits::coll::AS_SLICE;

pub fn gw_mod_bf<T>(
    src: &T,
    map_ind: &FxHashMap<&'static str, T>,
    settings: &'static Vec<SETTINGS_USED_MODS>,
    map_args_: &MAP_ARGS<T, T>,
    map_mod_t_bf_: &MAP_MOD_T_BF<T, T>,
    bf: &mut MAP_BF_VEC<T>,
) -> T
where 
    T: Float,
{
    settings
        .iter()
        .fold(
            *src,
            |src_iter, setting| {
                let key = setting.key.as_str();
                map_mod_t_bf_[key](
                    &src_iter,
                    setting.used_indications
                        .iter()
                        .map(|usd_ind| map_ind[usd_ind.as_str()])
                        .collect::<Vec<T>>()
                        .as_slice(),
                    map_args_.get(key).unwrap(),
                    bf.get_mut(key).unwrap(),
                )
            }
        )
}

pub fn gw_mod_f<T>(
    ind: &T,
    map_t: &FxHashMap<&'static str, T>,
    settings: &Vec<SETTINGS_USED_MODS>,
    map_args_: &MAP_ARGS<T, T>,
    map_mod_t_: &MAP_MOD_T<T, T>,
) -> T
where 
    T: Float,
{
    settings
        .iter()
        .fold(
            *ind,
            |ind_iter, setting| {
                let key = setting.key.as_str();
                map_mod_t_.get(key).unwrap()(
                    &ind_iter,
                    setting.used_indications
                        .iter()
                        .map(|usd_ind| map_t[usd_ind.as_str()])
                        .collect::<Vec<T>>()
                        .as_slice(),
                    map_args_.get(key).unwrap(),
                )
            }
        )
}

pub fn gw_mod_coll<C, T>(
    init_ind: C,
    map_ind: &FxHashMap<&'static str, C>,
    settings: &Vec<SETTINGS_USED_MODS>,
    map_args_: &MAP_ARGS<T, T>,
    map_mod_coll_: &MAP_MOD_COLL<C, T, T>,
) -> C
where 
    T: Float,
    C: FromIterator<T>,
    C: IntoIterator<Item = T>,
    C: AS_SLICE<T>
{
    settings
        .iter()
        .fold(
            init_ind,
            |ind_iter, setting| {
                let key = setting.key.as_str();
                map_mod_coll_[key](
                    ind_iter.as_slice(),
                    setting.used_indications
                        .iter()
                        .map(|usd_ind| map_ind[usd_ind.as_str()].as_slice())
                        .collect::<Vec<&[T]>>()
                        .as_slice(),
                    map_args_.get(key).unwrap(),
                )
            }
        )
        .into_iter()
        .collect()
}