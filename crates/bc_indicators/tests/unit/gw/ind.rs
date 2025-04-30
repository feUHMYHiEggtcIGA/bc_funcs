use bc_indicators::ind::no_osc::other::avg::avg_coll;
use bc_utils::nums::avg;
use bc_utils_lg::statics::settings::{
    SETTINGS_RSI_EMPTY, 
    SETTINGS_IND_TEST,
};
use bc_utils_lg::statics::prices::{
    HIGH, 
    LOW,
    OPEN,
    SRCS,
};

use bc_indicators::gw::bf::gw_func_bf_ind;
use bc_indicators::map::bf::map_func_bf_ind;
use bc_indicators::gw::ind::*;
use bc_indicators::map::ind::*;
use bc_indicators::map::args::*;
use bc_indicators::ind::osc::other::rsi::{rsi_coll, rsi_f};


#[test]
fn gw_ind_bf_res_sett_rsi_empty_1() {
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
    assert_eq!(
        gw_ind_bf(
            &SRCS, 
            &SETTINGS_RSI_EMPTY, 
            &map_ind_bf_,
            &map_args_, 
            &mut bf,
        )["rsi_1"],
        rsi_f(OPEN.as_slice(), &SETTINGS_RSI_EMPTY["rsi_1"].kwargs_usize["window"]),
    )
}

#[test]
fn gw_ind_bf_res_sett_test_1() {
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
    let src_test = avg_coll::<Vec<f64>, _,>(&[
        OPEN.as_slice(),
        &LOW[..LOW.len() - 1],
        &HIGH[..HIGH.len() - 1],
    ]);
    let rsi_1 = rsi_coll::<Vec<f64>, _>(src_test.as_slice(), &SETTINGS_IND_TEST["rsi_1"].kwargs_usize["window"]);
    let rsi_2 =  rsi_f(rsi_1.as_slice(), &SETTINGS_IND_TEST["rsi_2"].kwargs_usize["window"]);
    let map_ind_bf_ = map_ind_t_bf();
    assert!(
        (gw_ind_bf(
            &SRCS, 
            &SETTINGS_IND_TEST, 
            &map_ind_bf_,
            &map_args_, 
            &mut bf,
        )["ind"] / 
        avg::<f64, _>(&[&OPEN[OPEN.len() - 2], &rsi_2]) - 1.0).abs() < 0.0001,
    );
}