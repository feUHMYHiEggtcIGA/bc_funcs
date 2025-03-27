#[allow(clippy::wildcard_imports)]
use bc_settings::settings::SETTINGS;

use bc_indicators::indicators_gateway::*;
use bc_indicators::rm_gateway::*;
use bc_utils_lg::statics::prices::{SRC_EL, SRC};

#[test]
fn indicators_gw_rm_1() {
    let mut rm = rm_gw(
        &SRC, 
        &SETTINGS.indications
    );
    let map_args = map_args_rm(&SETTINGS.indications);
    let map_indicators = map_indicators_rm(&SETTINGS.indications);
    assert_eq!(
        indications_gw_rm(
            &SRC_EL, 
            &SETTINGS.indications, 
            &mut rm,
            &map_args,
            &map_indicators,
        ).get("rsi_1").expect("indication not found"),
        &40.41131222134466,
    );
}

#[test]
fn args_map_1() {
    let map_args = map_args_rm(&SETTINGS.indications);
    assert!(map_args.contains_key("rsi_1"));
}

#[test]
fn args_rm_1() {
    let map_indicators = map_indicators_rm(&SETTINGS.indications);
    assert!(map_indicators.contains_key("rsi"));
}