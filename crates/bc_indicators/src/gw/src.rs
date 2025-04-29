use num_traits::Float;
use bc_utils_lg::structs::settings::{SETTINGS_IND, SETTINGS_USED_SRC};
use bc_utils_lg::types::structures::*;
use bc_utils_lg::types::maps::*;


pub fn src_from_settings<T>(
    used_ind: &Vec<String>,
    used_src: &Vec<SETTINGS_USED_SRC>,
    settings: &'static MAP_LINK<String, SETTINGS_IND>,
    src: &SRCS<T>,
    map_ind_coll_: &MAP_IND_COLL<Vec<T>, T>,
    map_args_ind_bf_: &MAP_ARGS<T>,
) -> Vec<Vec<T>>
where 
    T: Float,
{
    let mut res = vec![];
    for us in used_ind
    {
        res.push(
            map_ind_coll_[settings[us.as_str()].key.as_str()](
                // recursive func
                src_from_settings::<T>(
                    &settings[us].used_ind, 
                    &settings[us].used_src, 
                    settings, 
                    src, 
                    map_ind_coll_,
                    map_args_ind_bf_,
                )
                    .iter()
                    .map(Vec::as_slice)
                    .collect::<Vec<&[T]>>()
                    .as_slice(),
                &map_args_ind_bf_[us.as_str()]
            )
        );
    }
    for us_el in used_src
    {
        res.push({
            let sk = &src[&us_el.key];
            sk[..sk.len() - us_el.sub_from_last_i].to_vec()
        });
    }
    res
}
