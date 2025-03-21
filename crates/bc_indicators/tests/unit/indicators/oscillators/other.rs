use super::super::super::super::common;
use bc_indicators::indicators::oscillators::other::*;
use bc_indicators::rm;

#[test]
fn rsi_rm_1() {
    let vec = common::PRICES;
    let (
        mut rm, 
        mut rm_rma1, 
        mut rm_rma2
    ) = rm::rm_rsi(
        vec.as_slice(),
        &common::WINDOW,
    );

    assert_eq!(
        rsi_rm(
            &2.2547, 
            &mut rm, 
            &mut rm_rma1, 
            &mut rm_rma2,
        ), 
        40.41131222134466,
    )
}

#[test]
fn rsi_float_1() {
    let vec = common::PRICES;

    assert_eq!(
        rsi_float(vec.as_slice(),  &2,),
        40.41131222134466,
    )
}