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
    OPEN_LAST, 
    SRCS,
};

use bc_indicators::gw::bf::gw_func_bf_ind;
use bc_indicators::map::bf::map_func_bf_ind;
use bc_indicators::gw::ind::*;
use bc_indicators::map::ind::*;
use bc_indicators::map::args::*;
use bc_indicators::ind::osc::other::rsi::rsi_f;


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
        40.410730678054115,
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
    let map_ind_bf_ = map_ind_t_bf();
    assert_eq!(
        gw_ind_bf(
            &SRCS, 
            &SETTINGS_IND_TEST, 
            &map_ind_bf_,
            &map_args_, 
            &mut bf,
        )["src_1"],
        avg(&[OPEN_LAST, HIGH[HIGH.len() - 2], LOW[LOW.len() - 2], ]),
    )
}

#[test]
fn gw_ind_bf_res_sett_test_2() {
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
    let src = avg_coll::<Vec<f64>, _,>(&[
        OPEN.as_slice(),
        &LOW[1..LOW.len() - 1],
        &HIGH[1..HIGH.len() - 1],
    ]);
    assert_eq!(
        gw_ind_bf(
            &SRCS, 
            &SETTINGS_IND_TEST, 
            &map_ind_bf_,
            &map_args_, 
            &mut bf,
        )["rsi_1"],
        rsi_f(src.as_slice(), &SETTINGS_IND_TEST["rsi_1"].kwargs_usize["window"]),
    )
}

#[test]
fn gw_ind_bf_res_sett_test_3() {
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
    let src = avg_coll::<Vec<f64>, _,>(&[
        OPEN.as_slice(),
        &LOW[1..LOW.len() - 1],
        &HIGH[1..HIGH.len() - 1],
    ]);
    assert_eq!(
        gw_ind_bf(
            &SRCS, 
            &SETTINGS_IND_TEST, 
            &map_ind_bf_,
            &map_args_, 
            &mut bf,
        )["rsi_3"],
        avg(&[
            rsi_f(src.as_slice(), &SETTINGS_IND_TEST["rsi_1"].kwargs_usize["window"]),
            rsi_f(src.as_slice(), &SETTINGS_IND_TEST["rsi_2"].kwargs_usize["window"]),
        ]),
    )
}