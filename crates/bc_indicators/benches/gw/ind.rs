use criterion::{
    Criterion, 
    criterion_group,
    criterion_main, 
};
use bc_utils_lg::statics::settings::{
    SETTINGS_RSI_EMPTY, 
    SETTINGS_IND_TEST,
};
use bc_utils_lg::statics::prices::SRCS;

use bc_indicators::gw::bf::gw_func_bf_ind;
use bc_indicators::map::bf::map_func_bf_ind;
use bc_indicators::gw::ind::*;
use bc_indicators::map::ind::*;
use bc_indicators::map::args::*;


fn gw_ind_bf_sett_rsi_empty_1(m: &mut Criterion) {
    let map_func_bf_ind_ = map_func_bf_ind();
    let map_ind_coll_ = map_ind_coll();
    let map_args_ = map_args_ind(&SETTINGS_RSI_EMPTY);
    let mut bf = gw_func_bf_ind(
        &SRCS,
        &SETTINGS_RSI_EMPTY,
        &map_func_bf_ind_, 
        &map_ind_coll_, 
        &map_args_, 
        &true,
    );
    let map_ind_bf_ = map_ind_t_bf();
    m.bench_function(
        "gw_ind_bf_sett_rsi_empty_1", 
        |f| f.iter(||
            gw_ind_bf(
                &SRCS, 
                &SETTINGS_RSI_EMPTY, 
                &map_ind_bf_,
                &map_args_, 
                &mut bf,
            )
    ));
}

fn gw_ind_bf_sett_ind_test_1(m: &mut Criterion) {
    let map_func_bf_ind_ = map_func_bf_ind();
    let map_ind_coll_ = map_ind_coll();
    let map_args_ = map_args_ind(&SETTINGS_IND_TEST);
    let mut bf = gw_func_bf_ind(
        &SRCS,
        &SETTINGS_IND_TEST,
        &map_func_bf_ind_, 
        &map_ind_coll_, 
        &map_args_, 
        &true,
    );
    let map_ind_bf_ = map_ind_t_bf();
    m.bench_function(
        "gw_ind_bf_sett_ind_test_1", 
        |f| f.iter(||
            gw_ind_bf(
                &SRCS, 
                &SETTINGS_IND_TEST, 
                &map_ind_bf_,
                &map_args_, 
                &mut bf,
            )
    ));
}

criterion_group!(
    benches, 
    gw_ind_bf_sett_rsi_empty_1, 
    gw_ind_bf_sett_ind_test_1,
);
criterion_main!(benches);