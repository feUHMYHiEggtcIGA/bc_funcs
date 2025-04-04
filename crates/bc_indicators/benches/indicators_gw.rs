use criterion::{
    Criterion, 
    black_box, 
    criterion_group,
    criterion_main, 
};
use bc_utils_lg::statics::settings::SETTINGS_IND_TEST;

use bc_indicators::rm_gateway::*;
use bc_indicators::indicators_gateway::*;
use bc_indicators::mods_gateway::*;
use bc_utils_lg::statics::prices::{SRC, SRC_EL};

fn ind_gw_rm_settings_ind_test_1(m: &mut Criterion) {
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
    m.bench_function(
        "ind_gw_rm_settings_ind_test_1", 
        |f| f.iter(||
            gw_ind_rm_abstr(
                &SRC_EL, 
                &SETTINGS_IND_TEST, 
                &map_ind_t_rm_abstr_,
                &map_mod_all_abstr_,
                &map_args_ind_rm_,
                &map_map_args_mod_,
                &mut rm,
                &mut map_map_rm_mods,
            )
    ));
}

criterion_group!(benches, ind_gw_rm_settings_ind_test_1);
criterion_main!(benches);