#[allow(clippy::wildcard_imports)]
use bc_settings::settings::SETTINGS;

use bc_indicators::indicators_gateway::*;
use bc_indicators::rm_gateway::*;
use bc_indicators::common::{SRC, SRC_EL};

#[test]
fn indicators_gw_rm_1() {
    let mut rm = rm_gw(
        &SRC, 
        &SETTINGS.indications
    );
    map_args_rm_init(&SETTINGS.indications);
    map_indicators_rm_init(&SETTINGS.indications);
    assert_eq!(
        indications_gw_rm(&SRC_EL, &SETTINGS.indications, &mut rm).get("rsi_1").expect("indication not found"),
        &40.41131222134466,
    );
}