use bc_utils_lg::enums::indicators::T_ARGS;
use bc_utils_lg::statics::settings::SETTINGS_RSI_EMPTY;
use bc_utils_lg::statics::prices::{SRCS, OPEN_LAST};
use bc_utils_lg::implement::structures::VEC;

use bc_indicators::map::{
    ind::map_ind_t_bf,
    bf::map_func_bf_ind,
    args::*,
    mods::map_mod_coll,
};
use bc_indicators::gw::bf::gw_func_bf_ind;


#[test]
fn map_ind_bf_res_rsi_1(){
    let map_func_bf_ind_ = map_func_bf_ind();
    let map_args_ind_bf_ = map_args_ind(&SETTINGS_RSI_EMPTY);
    let map_mod_coll_ = map_mod_coll::<VEC<f64>, f64>();
    let map2_args_mods_src_ = map2_args_mods_src(&SETTINGS_RSI_EMPTY);
    let map_ind_bf_ = map_ind_t_bf();
    
    let mut bf = gw_func_bf_ind(
        &SRCS, 
        &SETTINGS_RSI_EMPTY,
        &map_func_bf_ind_,
        &map_args_ind_bf_,
        &true,
        &map_mod_coll_,
        &map2_args_mods_src_,
    );
    assert_eq!(
        map_ind_bf_["rsi"](&[OPEN_LAST], &vec![T_ARGS::Usize(2)], bf.get_mut("rsi_1").unwrap()),
        40.410730678054115,
    )
}