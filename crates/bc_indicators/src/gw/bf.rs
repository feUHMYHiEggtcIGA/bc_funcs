use std::ops::{
    AddAssign,
    DivAssign,
};
use std::iter::Sum;

use num_traits::Float;
use bc_utils_lg::types::structures::*;
use bc_utils_lg::types::maps::*;
use bc_utils_lg::structs::settings::SETTINGS_IND;

use crate::gw::src::*;


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
#[must_use]
pub fn gw_func_bf_ind<'a, T>(
    src: &SRCS<T>,
    settings: &'static MAP_LINK<String, SETTINGS_IND>,
    map_bf_ind_abstr_: &'a MAP_FUNC_BF_IND<T>,
    map_ind_coll_: &MAP_IND_COLL<Vec<T>, T>,
    map_args_ind_bf_: &MAP_ARGS<T>,
    exc_last: &bool,
) -> MAP_BF_VEC<'a, T>
where 
    T: Float,
    T: Sum,
    T: AddAssign,
    T: DivAssign,
{   
    settings
        .iter()
        .map(
            |setting,|
            {
                let key_uniq = setting.0.as_str();
                (
                    key_uniq,
                    map_bf_ind_abstr_
                        [setting.1.key.as_str()]
                        (
                            src_from_settings(
                                &setting.1.used_ind, 
                                &setting.1.used_src, 
                                settings, 
                                src, 
                                map_ind_coll_, 
                                map_args_ind_bf_,
                            )
                                .iter()
                                .map(Vec::as_slice)
                                .collect::<Vec<&[T]>>()
                                .as_slice(),
                            &map_args_ind_bf_[key_uniq],
                            exc_last,
                    )
                )
            }
        )
        .collect()
}
