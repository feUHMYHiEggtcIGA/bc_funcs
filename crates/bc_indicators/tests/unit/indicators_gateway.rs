use bc_indicators::indicators::no_oscillators::trend::sma_coll;
use bc_indicators::indicators::oscillators::other::rsi_f;
use bc_indicators::indicators::no_oscillators::trend::sma_f;
use bc_indicators::indicators_gateway::*;
use bc_indicators::mods::nohesi_f;
use bc_indicators::mods_gateway::*;
use bc_indicators::rm_gateway::*;
use bc_utils_lg::statics::prices::{SRC_EL, SRC};
use bc_utils_lg::statics::settings::SETTINGS_IND_TEST;

#[test]
fn indicators_gw_rm_1() {
    let map_ind_t_rm_abstr_ = map_ind_t_rm_abstr();
    let map_args_ind_rm_ = map_args_ind(&SETTINGS_IND_TEST);
    let map_map_args_mod_ = map_map_args_mod(&SETTINGS_IND_TEST);
    let map_mod_all_abstr_ = map_mod_all_abstr();
    let map_func_rm_ind_abstr_ = map_func_rm_ind_abstr();
    let map_func_rm_mod_abstr_ = map_func_rm_mod_abstr();
    let map_func_window_ind_abstr_ = map_func_window_func_rm_abstr();
    let map_ind_coll_abstr_ = map_ind_coll_abstr();
    let map_windows_ind = gw_map_func_window_ind_abstr(
        &SETTINGS_IND_TEST, 
        &map_func_window_ind_abstr_,
        &map_args_ind_rm_,
    );
    let mut rm = gw_func_rm_ind_abstr(
        &SRC, 
        &SETTINGS_IND_TEST,
        &map_func_rm_ind_abstr_,
        &map_args_ind_rm_,
        &true,
    );
    let mut map_map_rm_mods = gw_func_rm_mods(
        &SRC, 
        &SETTINGS_IND_TEST, 
        &map_ind_coll_abstr_, 
        &map_func_rm_mod_abstr_, 
        &map_windows_ind, 
        &map_args_ind_rm_, 
        &map_map_args_mod_,
        &true,
    );
    assert_eq!(
        gw_ind_rm_abstr(
            &SRC_EL, 
            &SETTINGS_IND_TEST, 
            &map_ind_t_rm_abstr_,
            &map_mod_all_abstr_,
            &map_args_ind_rm_,
            &map_map_args_mod_,
            &mut rm,
            &mut map_map_rm_mods,
        ).get("rsi_1").expect("indication not found"),
        &40.410730678054115,
    );
}

#[test]
fn args_map_1() {
    let map_args = map_args_ind::<f64>(&SETTINGS_IND_TEST);
    assert!(map_args.contains_key("rsi_1"));
}

#[test]
fn ind_gw_rm_rsi_avg_1() {
    let map_ind_t_rm_abstr_ = map_ind_t_rm_abstr();
    let map_args_ind_rm_ = map_args_ind(&SETTINGS_IND_TEST);
    let map_map_args_mod_ = map_map_args_mod(&SETTINGS_IND_TEST);
    let map_mod_all_abstr_ = map_mod_all_abstr();
    let map_func_rm_ind_abstr_ = map_func_rm_ind_abstr();
    let map_func_rm_mod_abstr_ = map_func_rm_mod_abstr();
    let map_func_window_ind_abstr_ = map_func_window_func_rm_abstr();
    let map_ind_coll_abstr_ = map_ind_coll_abstr();
    let map_windows_ind = gw_map_func_window_ind_abstr(
        &SETTINGS_IND_TEST, 
        &map_func_window_ind_abstr_,
        &map_args_ind_rm_,
    );
    let mut rm = gw_func_rm_ind_abstr(
        &SRC, 
        &SETTINGS_IND_TEST,
        &map_func_rm_ind_abstr_,
        &map_args_ind_rm_,
        &true,
    );
    let mut map_map_rm_mods = gw_func_rm_mods(
        &SRC, 
        &SETTINGS_IND_TEST, 
        &map_ind_coll_abstr_, 
        &map_func_rm_mod_abstr_, 
        &map_windows_ind, 
        &map_args_ind_rm_, 
        &map_map_args_mod_,
        &true,
    );
    let map_res = gw_ind_rm_abstr(
        &SRC_EL, 
        &SETTINGS_IND_TEST, 
        &map_ind_t_rm_abstr_,
        &map_mod_all_abstr_,
        &map_args_ind_rm_,
        &map_map_args_mod_,
        &mut rm,
        &mut map_map_rm_mods,
    );
    let res = map_res.get("rsi_2").expect("indication not found");
    let res_test = (rsi_f(SRC.open, &3) + rsi_f(SRC.open, &2)) / 2.0;
    assert_eq!(res, &38.043589495370526);
    assert_eq!(res, &res_test);
}

#[test]
fn ind_gw_rm_sma_nohesi_1() {
    let map_ind_t_rm_abstr_ = map_ind_t_rm_abstr();
    let map_args_ind_rm_ = map_args_ind(&SETTINGS_IND_TEST);
    let map_map_args_mod_ = map_map_args_mod(&SETTINGS_IND_TEST);
    let map_mod_all_abstr_ = map_mod_all_abstr();
    let map_func_rm_ind_abstr_ = map_func_rm_ind_abstr();
    let map_func_rm_mod_abstr_ = map_func_rm_mod_abstr();
    let map_func_window_ind_abstr_ = map_func_window_func_rm_abstr();
    let map_ind_coll_abstr_ = map_ind_coll_abstr();
    let map_windows_ind = gw_map_func_window_ind_abstr(
        &SETTINGS_IND_TEST, 
        &map_func_window_ind_abstr_,
        &map_args_ind_rm_,
    );
    let mut rm = gw_func_rm_ind_abstr(
        &SRC, 
        &SETTINGS_IND_TEST,
        &map_func_rm_ind_abstr_,
        &map_args_ind_rm_,
        &true,
    );
    let mut map_map_rm_mods = gw_func_rm_mods(
        &SRC, 
        &SETTINGS_IND_TEST, 
        &map_ind_coll_abstr_, 
        &map_func_rm_mod_abstr_, 
        &map_windows_ind, 
        &map_args_ind_rm_, 
        &map_map_args_mod_,
        &true,
    );
    let map_res = gw_ind_rm_abstr(
        &SRC_EL, 
        &SETTINGS_IND_TEST, 
        &map_ind_t_rm_abstr_,
        &map_mod_all_abstr_,
        &map_args_ind_rm_,
        &map_map_args_mod_,
        &mut rm,
        &mut map_map_rm_mods,
    );
    let res = map_res.get("sma_1").expect("indication not found");
    let res_test = &nohesi_f(sma_coll::<Vec<f64>, f64>(SRC.open, &2).as_slice(), &0.0001);
    assert_eq!(res, &2.2544500000000003);
    assert_eq!(res, res_test);
}