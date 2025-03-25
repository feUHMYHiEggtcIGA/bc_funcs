use std::sync::LazyLock;

use criterion::{
    Criterion, 
    black_box, 
    criterion_group,
    criterion_main, 
};
use bc_utils_lg::structs::settings::SettingsInd;
use rustc_hash::FxHashMap;

use bc_indicators::rm_gateway::*;
use bc_indicators::indicators_gateway::*;
use bc_indicators::common::{SRC, SRC_EL};

static SETTINGS_IND_BENCH: LazyLock<Vec<SettingsInd>> = LazyLock::new(|| vec![SettingsInd{
    key: String::from("rsi"),
    key_uniq: String::from("rsi_1"),
    kwargs_usize: FxHashMap::from_iter([(String::from("window"), 2)]),
    kwargs_f64: FxHashMap::default(),
    kwargs_string: FxHashMap::default(),
    used_src: vec![],
    used_indications: vec![],
    used_mods: vec![],
    execution_on_count: 0,
}]);

fn indicators_gw_rm_1(m: &mut Criterion) {
    map_args_rm_init(&SETTINGS_IND_BENCH);
    map_indicators_rm_init(&SETTINGS_IND_BENCH);
    let mut rm = rm_gw(
        &SRC,
        &SETTINGS_IND_BENCH
    );
    m.bench_function(
        "indicators_gw_rm_1", 
        |f| f.iter(||
            indications_gw_rm(
                &SRC_EL, 
                &SETTINGS_IND_BENCH, 
                black_box(&mut rm)
            )
    ));
}

criterion_group!(benches, indicators_gw_rm_1);
criterion_main!(benches);