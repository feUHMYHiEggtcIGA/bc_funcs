use bc_indicators::indicators_gateway;
use bc_indicators::indicators_gateway::*;
use bc_indicators::rm_gateway::*;
use bc_utils_lg::statics::prices::{SRC_EL, SRC};
use bc_utils_lg::statics::settings::SETTINGS_IND_TEST;

#[test]
fn indicators_gw_rm_1() {
    let mut rm = rm_gw(
        &SRC, 
        &SETTINGS_IND_TEST
    );
    let map_args = map_args_rm(&SETTINGS_IND_TEST);
    let map_indicators = map_indicators_rm(&SETTINGS_IND_TEST);
    assert_eq!(
        indicators_gw_rm(
            &SRC_EL, 
            &SETTINGS_IND_TEST, 
            &mut rm,
            &map_args,
            &map_indicators,
        ).get("rsi_1").expect("indication not found"),
        &40.41131222134466,
    );
}

#[test]
fn args_map_1() {
    let map_args = map_args_rm::<f64>(&SETTINGS_IND_TEST);
    assert!(map_args.contains_key("rsi_1"));
}

#[test]
fn args_rm_1() {
    let map_indicators = map_indicators_rm::<f64>(&SETTINGS_IND_TEST);
    assert!(map_indicators.contains_key("rsi"));
}

#[test]
fn indication_used_from_map_1() {
    let mut rm = rm_gw(&SRC, &SETTINGS_IND_TEST);
    let map_args = map_args_rm(&SETTINGS_IND_TEST);
    let map_indicators = map_indicators_rm(&SETTINGS_IND_TEST);
    assert_eq!(
        map_indicators.get("rsi").expect("ind not found")(
            &SRC_EL,
            map_args.get("rsi_1").expect("args not found in map"),
            rm.get_mut("rsi_1").expect("rm not found in map"),
        ),
        40.41131222134466
    );
}