use bc_utils_lg::statics::prices::OPEN;
use bc_utils_lg::statics::settings::WINDOW;
use bc_indicators::ind::no_osc::trend::ema::*;
use bc_indicators::bf::ema::bf_ema;


#[test]
fn ema_bf_res_1() {
    let mut bf = bf_ema(
        OPEN.as_slice(),
        &WINDOW,
        &true,
    );

    assert_eq!(
        ema_bf(&2.2547, &mut bf),
        2.254711084891796
    );
}

#[test]
fn ema_bf_res_skip_1() {
    let mut bf = bf_ema(
        &OPEN[2..],
        &WINDOW,
        &true,
    );

    assert_eq!(
        ema_bf(&2.2547, &mut bf),
        2.254711084891796
    );
}

#[test]
fn ema_f_res_1() {
    assert_eq!(
        ema_f(OPEN.as_slice(), &WINDOW),
        2.254711084891796,
    );
}

#[test]
fn ema_f_res_skip_1() {
    assert_eq!(
        ema_f(&OPEN[2..], &WINDOW),
        2.254711084891796,
    );
}

#[test]
fn ema_coll_res_1() {
    assert!(
        (ema_coll::<Vec<f64>, _,>(OPEN.as_slice(), &WINDOW).last().unwrap() /
        &ema_f(OPEN.as_slice(), &WINDOW) - 1.0).abs() < 0.0001
    );
}