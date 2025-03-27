use bc_utils_lg::statics::prices::SRC;
use bc_utils_lg::statics::settings::SETTINGS_IND_TEST;

use bc_indicators::rm_gateway::rm_gw;

#[test]
fn rm_contains_1() {
    assert!(rm_gw(&SRC, &SETTINGS_IND_TEST).contains_key("rsi_1"))
}