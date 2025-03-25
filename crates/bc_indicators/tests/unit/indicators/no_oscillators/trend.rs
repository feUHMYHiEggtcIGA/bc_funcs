use bc_indicators::common;
use bc_indicators::indicators::no_oscillators::trend::*;
use bc_indicators::rm;


#[test]
fn alpha_ema_1() { assert_eq!(alpha_ema::<f64>(&9.0), 0.2) }

#[test]
fn alpha_rma_1() { assert_eq!(alpha_rma(&10.0), 0.1,) }

#[test]
fn ema_rm_1() {
    let mut rm = rm::rm_ema(
        common::OPEN.as_slice(),
        &common::WINDOW,
    );

    assert_eq!(
        ema_rm(2.2547, &mut rm),
        2.2547110887999526
    );
}

#[test]
fn ema_rm_skip_1() {
    let mut rm = rm::rm_ema(
        &common::OPEN[2..],
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
        ema_float(common::OPEN.as_slice(), &common::WINDOW),
        2.2547110887999526,
    );
}

#[test]
fn ema_float_skip_1() {
    assert_eq!(
        ema_float(&common::OPEN[2..], &common::WINDOW),
        2.2547110887999526,
    );
}

#[test]
fn rma_rm_1() {
    let mut rm = rm::rm_rma(
        common::OPEN.as_slice(),
        &common::WINDOW,
    );

    assert_eq!(
        rma_rm(&2.2547, &mut rm),
        2.2548923400878906,
    );
}

#[test]
fn rma_rm_skip_() {
    let mut rm = rm::rm_rma(
        &common::OPEN[2..],
        &common::WINDOW,
    );

    assert_eq!(
        rma_rm(&2.2547, &mut rm),
        2.2548923400878906,
    );
}

#[test]
fn sma_rm_1() {
    let mut rm = rm::rm_sma(
        common::OPEN.as_slice(),
        &common::WINDOW,
    );
    assert_eq!(
        sma_rm::<f64, &f64>(&2.2547, &common::WINDOW, &mut rm),
        2.2544500000000003,
    );
}

#[test]
fn sma_rm_skip_1() {
    let mut rm = rm::rm_sma(
        &common::OPEN[2..],
        &common::WINDOW,
    );
    assert_eq!(
        sma_rm::<f64, &f64>(&2.2547, &common::WINDOW, &mut rm),
        2.2544500000000003,
    );
}