use bc_indicators::indicators_gateway::map_args_ind;
use bc_utils_lg::statics::prices::SRC;
use bc_utils_lg::statics::settings::SETTINGS_IND_TEST;

use bc_indicators::rm_gateway::*;

#[test]
fn rm_contains_1() {
    let map_rm = map_func_rm_ind_abstr();
    let map_args = map_args_ind(&SETTINGS_IND_TEST);
    assert!(gw_func_rm_ind_abstr(
        &SRC, 
        &SETTINGS_IND_TEST,
        &map_rm,
        &map_args,
        &true,
    ).contains_key("rsi_1"))
}