use super::super::super::super::common;
use bc_indicators::indicators::no_oscillators::trend::*;
use bc_indicators::rm;


#[test]
fn alpha_ema_1() { assert_eq!(alpha_ema::<f64>(&9.0), 0.2) }

#[test]
fn alpha_rma_1() { assert_eq!(alpha_rma(&10.0), 0.1,) }

#[test]
fn ema_rm_1() {
    let vec = common::PRICES;
    let mut rm = rm::rm_ema(
        vec.as_slice(),
        &common::WINDOW,
    );

    assert_eq!(
        ema_rm(2.2547, &mut rm),
        2.2547110887999526
    );
}

#[test]
fn ema_float_1() {
    assert_eq!(
        ema_float(common::PRICES.as_slice(), &common::WINDOW),
        2.2547110887999526,
    );
}

#[test]
fn rma_rm_1() {
    let vec = common::PRICES;
    let mut rm = rm::rm_rma(
        vec.as_slice(),
        &common::WINDOW,
    );

    assert_eq!(
        rma_rm(&2.2547, &mut rm),
        2.2548923400878906,
    );
}

#[test]
fn sma_rm_1() {
    let vec = common::PRICES;
    let mut rm = rm::rm_sma(
        vec.as_slice(),
        &common::WINDOW,
    );
    assert_eq!(
        sma_rm::<f64, &f64>(&2.2547, &common::WINDOW, &mut rm),
        2.2544500000000003,
    );
}