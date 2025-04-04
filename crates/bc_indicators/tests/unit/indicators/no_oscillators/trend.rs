use bc_utils_lg::statics::prices::OPEN;
use bc_utils_lg::statics::settings::WINDOW;
use bc_indicators::indicators::no_oscillators::trend::*;
use bc_indicators::rm;


#[test]
fn alpha_ema_1() { assert_eq!(alpha_ema::<f64>(&9.0), 0.2) }

#[test]
fn alpha_rma_1() { assert_eq!(alpha_rma(&10.0), 0.1,) }

#[test]
fn ema_rm_1() {
    let mut rm = rm::rm_ema(
        OPEN.as_slice(),
        &WINDOW,
        &true,
    );

    assert_eq!(
        ema_rm(&2.2547, &mut rm),
        2.254711084891796
    );
}

#[test]
fn ema_rm_skip_1() {
    let mut rm = rm::rm_ema(
        &OPEN[2..],
        &WINDOW,
        &true,
    );

    assert_eq!(
        ema_rm(&2.2547, &mut rm),
        2.254711084891796
    );
}

#[test]
fn ema_f_1() {
    assert_eq!(
        ema_f(OPEN.as_slice(), &WINDOW),
        2.254711084891796,
    );
}

#[test]
fn ema_f_skip_1() {
    assert_eq!(
        ema_f(&OPEN[2..], &WINDOW),
        2.254711084891796,
    );
}

#[test]
fn rma_rm_1() {
    let mut rm = rm::rm_rma(
        OPEN.as_slice(),
        &WINDOW,
        &true,
    );

    assert_eq!(
        rma_rm(&2.2547, &mut rm),
        2.2548879972457887,
    );
}

#[test]
fn rma_rm_skip_() {
    let mut rm = rm::rm_rma(
        &OPEN[2..],
        &WINDOW,
        &true,
    );

    assert_eq!(
        rma_rm(&2.2547, &mut rm),
        2.2548879972457887,
    );
}

#[test]
fn sma_rm_1() {
    let mut rm = rm::rm_sma(
        OPEN.as_slice(),
        &WINDOW,
        &true,
    );
    assert_eq!(
        sma_rm::<f64>(2.2547, &WINDOW, &mut rm),
        2.2544500000000003,
    );
}

#[test]
fn sma_rm_skip_1() {
    let mut rm = rm::rm_sma(
        &OPEN[2..],
        &WINDOW,
        &true,
    );
    assert_eq!(
        sma_rm::<f64>(2.2547, &WINDOW, &mut rm),
        2.2544500000000003,
    );
}