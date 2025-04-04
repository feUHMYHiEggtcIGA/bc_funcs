use std::iter::Sum;
use std::ops::{AddAssign, DivAssign};

use bc_utils_lg::types::indicators::*;
use num_traits::Float;
use bc_utils_lg::structs::src::SRC;
use bc_utils_lg::structs::settings::SETTINGS_IND;
use bc_utils_lg::statics::funcs::*;
use bc_utils::transf::roll_slice1;

use crate::indicators::oscillators::trend::tqo_b;
use crate::indicators_gateway::gw_ind_coll_abstr;
// use crate::indicators_gateway::map_args_rm;
use crate::mods::nohesi_rm_abstr;
#[allow(clippy::wildcard_imports)]
use crate::rm::*;


pub fn map_func_rm_ind_abstr<'a, T>() -> MAP_aFUNC_RM_IND_ABSTR<'a, T>
where 
    T: Float,
    T: AddAssign,
    T: DivAssign,
{
    MAP_aFUNC_RM_IND_ABSTR::from_iter([
        ("sma", rm_sma_abstr as FUNC_RM_IND_aABSTR<T>),
        ("ema", rm_ema_abstr as FUNC_RM_IND_aABSTR<T>),
        ("rma", rm_rma_abstr as FUNC_RM_IND_aABSTR<T>),
        ("rsi", rm_rsi_abstr as FUNC_RM_IND_aABSTR<T>),
        ("tqo_b", rm_tqo_b_abstr as FUNC_RM_IND_aABSTR<T>),
    ])
}

pub fn map_func_rm_mod_abstr<'a, T>() -> MAP_aFUNC_RM_MOD_ABSTR<'a, T>
where 
    T: Float,
    T: AddAssign,
    T: DivAssign,
{
    MAP_aFUNC_RM_MOD_ABSTR::from_iter([
        ("nohesi", rm_nohesi_abstr as FUNC_RM_MOD_aABSTR<T>),
        ("avg", func_rm_mod_abstr_df as FUNC_RM_MOD_aABSTR<T>),
    ])
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
#[must_use]
pub fn gw_func_rm_ind_abstr<'a, T>(
    src: &SRC<T>,
    settings: &'static Vec<SETTINGS_IND>,
    map_rm_ind_abstr_: &'a MAP_aFUNC_RM_IND_ABSTR<T>,
    map_args_ind_rm_: &MAP_aARGS<T>,
    exc_last: &bool,
) -> MAP_aRM_VEC<'a, T>
where
    T: Float,
    T: AddAssign,
    T: DivAssign,
{   
    settings
        .iter()
        .map(
            |setting|
            {
                let key_uniq = setting.key_uniq.as_str();
                (
                    key_uniq,
                    map_rm_ind_abstr_
                    
                        .get(setting.key.as_str())
                        .expect("key rm not found")
                        (
                            src,
                            map_args_ind_rm_.get(key_uniq).unwrap(),
                            exc_last,
                    )
                )
            }
        )
        .collect()
}

pub fn gw_func_rm_mods<'a, T>(
    src: &SRC<T>,
    settings: &'static Vec<SETTINGS_IND>,
    map_ind_coll_abstr_: &MAP_aIND_COLL_ABSTR<Vec<T>, T>,
    map_funcs_rm_mods_abstr: &'a MAP_aFUNC_RM_MOD_ABSTR<T>,
    map_window_ind_: &MAP_aUSIZE,
    map_args_ind: &MAP_aARGS<T>,
    map_map_args_mods: &MAP_aMAP_ARGS<T>,
    exc_last: &bool,
) -> MAP_aMAP_RM_VEC<'a, T>
where 
    T: Float,
    T: Sum,
    T: AddAssign,
    T: DivAssign,
{
    let mut open = src.open.to_vec();
    let mut high = src.high.to_vec();
    let mut low = src.low.to_vec();
    let mut close = src.close.to_vec();
    settings
        .iter()
        .fold(
            (MAP_aMAP_RM_VEC::default(), MAP_aVEC::default()),
            |(mut res, mut map_vec_ind), setting,| {
                let key_uniq = setting.key_uniq.as_str();
                let shift = &2;
                // fix min 2 => mapping value format
                let range_window = src.open.len() - map_window_ind_[key_uniq] - 2..src.open.len();
                roll_slice1(open.as_mut_slice(), shift);
                roll_slice1(high.as_mut_slice(), shift);
                roll_slice1(low.as_mut_slice(), shift);
                roll_slice1(close.as_mut_slice(), shift);
                let src_rolled = SRC{
                    open: &open[range_window.clone()],
                    high: &high[range_window.clone()],
                    low: &low[range_window.clone()],
                    close: &close[range_window],
                };
                let ind = map_ind_coll_abstr_[setting.key.as_str()](
                    &src_rolled,
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
                                    map_funcs_rm_mods_abstr[key](
                                        ind.as_slice(),
                                        &mod_.used_indications
                                            .iter()
                                            .map(|ind_key_uniq| map_vec_ind[ind_key_uniq.as_str()].as_slice())
                                            .collect::<Vec<&[T]>>(),
                                        &map_map_args_mods[key_uniq][key],
                                        exc_last,
                                    )
                                )
                                
                            }
                        )
                    .collect::<MAP_aRM_VEC<T>>()
                );
                map_vec_ind.insert(key_uniq, ind);
                (res, map_vec_ind)
            }
        ).0
}