use bc_indicators::indicators_gateway::*;
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

// #[test]
// fn args_rm_1() {
//     let map_indicators = map_indicators_rm_abstr::<f64>();
//     assert!(map_indicators.contains_key("rsi"));
// }

// #[test]
// fn indication_used_from_map_1() {
//     let mut rm = rm_ind_abstr_gw(&SRC, &SETTINGS_IND_TEST);
//     let map_args = map_args_ind_rm(&SETTINGS_IND_TEST);
//     let map_indicators = map_indicators_rm_abstr();
//     assert_eq!(
//         map_indicators.get("rsi").expect("ind not found")(
//             &SRC_EL,
//             map_args.get("rsi_1").expect("args not found in map"),
//             rm.get_mut("rsi_1").expect("rm not found in map"),
//         ),
//         40.41131222134466
//     );
// }