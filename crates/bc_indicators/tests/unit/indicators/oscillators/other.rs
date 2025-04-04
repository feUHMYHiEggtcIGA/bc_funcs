use bc_utils_lg::statics::prices::OPEN;
use bc_utils_lg::statics::settings::WINDOW;
use bc_indicators::indicators::oscillators::other::*;
use bc_indicators::rm;

#[test]
fn rsi_rm_1() {
    let (
        mut rm, 
        mut rm_rma1, 
        mut rm_rma2
    ) = rm::rm_rsi(
        OPEN.as_slice(),
        &WINDOW,
        &true,
    );

    assert_eq!(
        rsi_rm(
            &2.2547, 
            &mut rm, 
            &mut rm_rma1, 
            &mut rm_rma2,
        ), 
        40.410730678054115,
    )
}

#[test]
fn rsi_rm_skip_1() {
    let (
        mut rm, 
        mut rm_rma1, 
        mut rm_rma2
    ) = rm::rm_rsi(
        &OPEN[2..],
        &WINDOW,
        &true,
    );

    assert_eq!(
        rsi_rm(
            &2.2547, 
            &mut rm, 
            &mut rm_rma1, 
            &mut rm_rma2,
        ), 
        40.410730678054115,
    )
}

#[test]
fn rsi_float_1() {
    assert_eq!(
        rsi_f(OPEN.as_slice(),  &2,),
        40.410730678054115,
    )
}

#[test]
fn rsi_float_skip_1() {
    assert_eq!(
        rsi_f(&OPEN[2..],  &2,),
        40.410730678054115,
    )
}