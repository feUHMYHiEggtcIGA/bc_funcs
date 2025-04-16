use bc_indicators::gw_abstr::src::{gw_src_coll, gw_src_f};
use bc_utils_lg::statics::settings::{
    SETTINGS_IND_TEST,
};
use bc_utils_lg::statics::prices::SRCS;
// use bc_utils_lg::implement::structures::VEC_F64;
// use bc_utils_lg::typ

use bc_indicators::map_abstr::mods::{map_mod_f, map_mod_coll};
use bc_indicators::map_abstr::args::*;


#[test]
fn gw_src_f_res_1(){
    let map_mod_ = map_mod_f();
    let map2_args_mods_src_ = map2_args_mods_src(&SETTINGS_IND_TEST);
    assert_eq!(
        gw_src_f(
            &SRCS, 
            &SETTINGS_IND_TEST[0].used_src, 
            &map_mod_, 
            &map2_args_mods_src_["rsi_1"], 
            Vec::new(),
            |v1, v2| v1.push(v2),
        ),
        vec![2.25385],
    )
}

// #[test]
// fn gw_src_coll_res_1(){
//     let map2_args_mods_src_ = map2_args_mods_src(&SETTINGS_IND_TEST);
//     let map_mod_coll_ = map_mod_coll::<VEC_F64, f64>();
//     assert_eq!(
//         gw_src_coll(
//             &SRCS, 
//             &SETTINGS_IND_TEST[0].used_src,
//             &map_mod_coll_,
//             &map2_args_mods_src_["rsi_1"],
//             VEC_F64(vec![]),
//             |v1, v2| v1.0.push(v2),
//         )   
//             .last()
//             .unwrap()
//             .last()
//             .unwrap(),
//         &2.25385,
//     );
// }