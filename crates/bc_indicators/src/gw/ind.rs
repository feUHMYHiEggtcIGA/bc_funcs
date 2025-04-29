use std::iter::Sum;
use std::ops::{
    AddAssign,
    DivAssign
};
use std::vec;

use num_traits::Float;
use bc_utils_lg::types::maps::{MAP, MAP_LINK};
use bc_utils_lg::types::maps::*;
use bc_utils_lg::types::structures::*;
use bc_utils_lg::structs::settings::SETTINGS_IND;


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::implicit_hasher)]
#[allow(clippy::ptr_arg)]
#[allow(clippy::too_many_arguments)]
pub fn gw_ind_bf<T>(
    buff_src: &SRCS<T>,
    settings: &'static MAP_LINK<String, SETTINGS_IND>,
    map_ind_bf_: &MAP_IND_T_BF<T>,
    map_args_ind_bf_: &MAP_ARGS<T>,
    map_bf_ind: &mut MAP_BF_VEC<T>,
) -> MAP<&'static str, T>
where 
    T: Float,
    T: Sum,
    T: AddAssign,
    T: DivAssign,
    // T: std::fmt::Debug,
{ 
    settings
        .iter()
        .fold(
            MAP::default(),
            |mut map, setting, | {
                let key_uniq_str = setting.0.as_str();
                let mut src_arg = vec![];
                for us_el in &setting
                    .1
                    .used_src
                {
                    src_arg.push({
                        let sk = &buff_src[&us_el.key];
                        sk[sk.len() - us_el.sub_from_last_i - 1]
                    });
                }
                for ui_el in &setting
                    .1
                    .used_ind
                {
                    // println!("{:#?} {}", map, ui_el.as_str());
                    src_arg.push(map[ui_el.as_str()]);
                }
                map.insert(
                    key_uniq_str,
                    map_ind_bf_[setting.1.key.as_str()](
                        src_arg.as_slice(),
                        &map_args_ind_bf_[key_uniq_str],
                        map_bf_ind.get_mut(key_uniq_str).unwrap(),
                    )
                );
                // println!("{map:#?}",);
                map
            }
        )
}