use core::f64;

use bc_indicators::gw_abstr::mods::gw_mod_coll;
use bc_utils::other::coll1_roll_replace_el;
use bc_utils_lg::enums::indicators::T_ARGS;
use bc_utils_lg::statics::settings::{
    SETTINGS_RSI_EMPTY, 
    SETTINGS_IND_TEST,
};
use bc_utils_lg::statics::prices::{CLOSE, SRCS};
use bc_utils_lg::types::maps_abstr::*;

use bc_indicators::gw_abstr::bf::gw_func_bf_ind;
use bc_indicators::gw_abstr::bf::gw_func_bf_mods;
use bc_indicators::map_abstr::bf::map_func_bf_ind;
use bc_indicators::map_abstr::bf::map_func_bf_mod;
use bc_indicators::map_abstr::mods::map_mod_all;
use bc_indicators::map_abstr::mods::map_mod_coll;
use bc_indicators::map_abstr::mods::map_mod_f;
use bc_indicators::gw_abstr::ind::*;
use bc_indicators::map_abstr::ind::*;
use bc_indicators::map_abstr::args::*;
use bc_utils_lg::structs::settings::SETTINGS_USED_MODS;
use rustc_hash::FxHashMap;


#[test]
fn gw_mod_coll_res_1(){
    let map_mod_coll_ = map_mod_coll();
    assert_eq!(
        gw_mod_coll(
            coll1_roll_replace_el::<Vec<f64>, _, _,>(CLOSE.clone().as_mut_slice(), &1, f64::NAN).as_slice(),
            &SRCS.iter().map(|v| (v.0.as_str(), v.1.clone())).collect(),
            &vec![
                SETTINGS_USED_MODS{
                    key: "avg".to_string(),
                    kwargs_usize: FxHashMap::default(),
                    kwargs_f64: FxHashMap::default(),
                    kwargs_string: FxHashMap::default(),
                    used_indications: vec!["open".to_string()]
                }
            ],
            &MAP_ARGS::from_iter([
                ("avg", vec![]),
            ]), 
            &map_mod_coll_,
        )
            .last()
            .unwrap(),
        &2.25385,
    )
}