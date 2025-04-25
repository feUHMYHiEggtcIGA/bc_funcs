use bc_utils_lg::statics::settings::{
    SETTINGS_RSI_EMPTY, 
    SETTINGS_IND_TEST,
};
use bc_utils_lg::statics::prices::SRCS;
use bc_utils_lg::structs_and_types::maps_abstr::MAP_COLL;
use bc_utils_lg::implement::structures::VEC;

use bc_indicators::gw::bf::gw_func_bf_ind;
use bc_indicators::gw::bf::gw_func_bf_mods;
use bc_indicators::map::bf::map_func_bf_ind;
use bc_indicators::map::bf::map_func_bf_mod;
use bc_indicators::map::mods::map_mod_all;
use bc_indicators::map::mods::map_mod_coll;
use bc_indicators::map::mods::map_mod_f;
use bc_indicators::gw::ind::*;
use bc_indicators::map::ind::*;
use bc_indicators::map::args::*;


#[test]
fn gw_ind_bf_res_rsi_1() {
    let map_ind_t_bf_ = map_ind_t_bf();
    let map_args_ind_bf_ = map_args_ind(&SETTINGS_RSI_EMPTY);
    let map1_args_mod_ = map1_args_mod(&SETTINGS_RSI_EMPTY);
    let map_mod_all_ = map_mod_all();
    let map_func_bf_ind_ = map_func_bf_ind();
    let map_func_bf_mod_ = map_func_bf_mod();
    let map_ind_coll_ = map_ind_coll::<VEC<f64>, _>();
    let map_mod_coll = map_mod_coll();
    let map_mod_f = map_mod_f();
    let map2_args_mods_src_ = map2_args_mods_src(
        &SETTINGS_RSI_EMPTY,
    );
    let mut bf = gw_func_bf_ind(
        &SRCS, 
        &SETTINGS_RSI_EMPTY,
        &map_func_bf_ind_,
        &map_args_ind_bf_,
        &true,
        &map_mod_coll,
        &map2_args_mods_src_,
    );
    let mut map2_bf_mods = gw_func_bf_mods(
        &SRCS, 
        &SETTINGS_RSI_EMPTY, 
        &map_ind_coll_,
        &map_func_bf_mod_,
        &map_args_ind_bf_,
        &map1_args_mod_,
        &map2_args_mods_src_,
        &true,
        &map_mod_coll,
        MAP_COLL::default(),
    );
    assert_eq!(
        gw_ind_bf(
            &SRCS, 
            &SETTINGS_RSI_EMPTY, 
            &map_ind_t_bf_,
            &map_mod_all_,
            &map_mod_f,
            &map_args_ind_bf_,
            &map1_args_mod_,
            &map2_args_mods_src_,
            &mut bf,
            &mut map2_bf_mods,
        )["rsi_1"],
        40.410730678054115,
    )
}

#[test]
fn gw_ind_bf_res_rsi_avg_1() {
    let map_ind_t_bf_ = map_ind_t_bf();
    let map_args_ind_bf_ = map_args_ind(&SETTINGS_IND_TEST);
    let map1_args_mod_ = map1_args_mod(&SETTINGS_IND_TEST);
    let map_mod_all_ = map_mod_all();
    let map_func_bf_ind_ = map_func_bf_ind();
    let map_func_bf_mod_ = map_func_bf_mod();
    let map_ind_coll_ = map_ind_coll::<VEC<f64>, _>();
    let map_mod_coll = map_mod_coll();
    let map_mod_f = map_mod_f();
    let map2_args_mods_src_ = map2_args_mods_src(
        &SETTINGS_IND_TEST,
    );
    let mut bf = gw_func_bf_ind(
        &SRCS, 
        &SETTINGS_IND_TEST,
        &map_func_bf_ind_,
        &map_args_ind_bf_,
        &true,
        &map_mod_coll,
        &map2_args_mods_src_,
    );
    let mut map2_bf_mods = gw_func_bf_mods(
        &SRCS, 
        &SETTINGS_IND_TEST, 
        &map_ind_coll_,
        &map_func_bf_mod_,
        &map_args_ind_bf_,
        &map1_args_mod_,
        &map2_args_mods_src_,
        &true,
        &map_mod_coll,
        MAP_COLL::default(),
    );
    assert_eq!(
        gw_ind_bf(
            &SRCS, 
            &SETTINGS_IND_TEST, 
            &map_ind_t_bf_,
            &map_mod_all_,
            &map_mod_f,
            &map_args_ind_bf_,
            &map1_args_mod_,
            &map2_args_mods_src_,
            &mut bf,
            &mut map2_bf_mods,
        )["rsi_2"],
        20.31680398083747,
    )
}