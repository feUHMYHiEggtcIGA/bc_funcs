use bc_utils_lg::statics::prices::*;
use bc_utils_lg::statics::settings::*;

use bc_indicators::ind::osc::other::rsi::*;
use bc_indicators::bf::rsi::bf_rsi;

#[test]
fn rsi_bf_res_1() {
    let (
        mut bf, 
        mut bf_rma1, 
        mut bf_rma2
    ) = bf_rsi(
        OPEN.as_slice(),
        &WINDOW,
        &true,
    );

    assert_eq!(
        rsi_bf(
            &2.2547, 
            &mut bf, 
            &mut bf_rma1, 
            &mut bf_rma2,
        ), 
        40.410730678054115 / 100.0,
    )
}

#[test]
fn rsi_bf_res_skip_1() {
    let (
        mut bf, 
        mut bf_rma1, 
        mut bf_rma2
    ) = bf_rsi(
        &OPEN[2..],
        &WINDOW,
        &true,
    );

    assert_eq!(
        rsi_bf(
            &2.2547, 
            &mut bf, 
            &mut bf_rma1, 
            &mut bf_rma2,
        ), 
        40.410730678054115 / 100.0,
    )
}

#[test]
fn rsi_f_res_1() {
    assert_eq!(
        rsi_f(OPEN.as_slice(),  &2,),
        40.410730678054115 / 100.0,
    )
}

#[test]
fn rsi_f_res_skip_1() {
    assert_eq!(
        rsi_f(&OPEN[2..],  &2,),
        40.410730678054115 / 100.0,
    )
}

#[test]
fn rsi_coll_res_1() {
    assert!(
        (rsi_coll::<Vec<f64>, _>(OPEN.as_slice(), &2).last().unwrap() / 
        &rsi_f(OPEN.as_slice(),  &2,) -1.0).abs() < 0.0001
    );
}