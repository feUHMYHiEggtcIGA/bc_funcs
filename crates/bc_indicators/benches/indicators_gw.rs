use criterion::{
    Criterion, 
    black_box, 
    criterion_group,
    criterion_main, 
};
use bc_utils_lg::statics::settings::SETTINGS_IND_TEST;

use bc_indicators::rm_gateway::*;
use bc_indicators::indicators_gateway::*;
use bc_utils_lg::statics::prices::{SRC, SRC_EL};

fn indicators_gw_rm_rsi_1(m: &mut Criterion) {
    let map_args = map_args_rm(&SETTINGS_IND_TEST);
    let map_indicators = map_indicators_rm(&SETTINGS_IND_TEST);
    let mut rm = rm_gw(
        &SRC,
        &SETTINGS_IND_TEST
    );
    m.bench_function(
        "indicators_gw_rm_rsi_1", 
        |f| f.iter(||
            indicators_gw_rm(
                &SRC_EL, 
                &SETTINGS_IND_TEST, 
                black_box(&mut rm),
                &map_args,
                &map_indicators,
            )
    ));
}

criterion_group!(benches, indicators_gw_rm_rsi_1);
criterion_main!(benches);