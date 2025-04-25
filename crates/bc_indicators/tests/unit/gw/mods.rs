use core::f64;

use bc_indicators::gw::mods::gw_mod_coll;
use bc_utils::other::coll1_roll_replace_el;
use bc_utils_lg::statics::prices::{CLOSE, SRCS};
use bc_utils_lg::structs_and_types::maps_abstr::*;
use bc_utils_lg::structs_and_types::settings::SETTINGS_USED_MODS;
use bc_utils_lg::implement::structures::VEC;
use rustc_hash::FxHashMap;

use bc_indicators::map::mods::map_mod_coll;


#[test]
fn gw_mod_coll_res_1(){
    let map_mod_coll_ = map_mod_coll::<VEC<f64>, f64,>();
    assert_eq!(
        gw_mod_coll::<VEC<f64>, f64>(
            VEC(coll1_roll_replace_el::<Vec<f64>, _, _,>(CLOSE.clone().as_mut_slice(), &1, f64::NAN)),
            &SRCS.iter().map(|v| (v.0.as_str(), VEC(v.1.clone()))).collect::<MAP_COLL<VEC<f64>>>(),
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
        ).0
            .last()
            .unwrap(),
        &2.25385,
    )
}