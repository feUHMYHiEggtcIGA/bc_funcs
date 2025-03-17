use std::collections::HashMap;

use bc_utils::transf;

use super::super::super::super::common;
use bc_indicators::indicators::no_oscillators::trend::*;
use bc_indicators::rm;


#[test]
fn alpha_ema_1() {
    assert_eq!(g_alpha_ema(&9.0), 0.2);
}

#[test]
fn alpha_rma_1() {
    assert_eq!(g_alpha_rma(&10.0),0.1,)
}

#[test]
fn ema_rm_1() {
    let vec = common::PRICES;
    let mut rm = rm::g_rm_ema(
        vec.iter(),
        &common::WINDOW,
    );

    assert_eq!(
        transf::g_round_float(g_ema_rm(&2.2547, &mut rm), 4),
        2.2547,
    );
}

#[test]
fn rma_rm_1() {
    let vec = common::PRICES;
    let mut rm = rm::g_rm_rma(
        vec.iter(),
        &vec.len(),
        &common::WINDOW,
    );

    assert_eq!(
        transf::g_round_float(g_rma_rm(&2.2547, &mut rm), 4),
        2.2549,
    );
}

#[test]
fn sma_rm_nolink_1() {
    let vec = common::PRICES;
    let mut rm = rm::g_rm_sma(
        vec.iter(),
        &vec.len(),
        &common::WINDOW,
    );
    assert_eq!(
        sma_rm::<f64, &f64>(&2.2547, &common::WINDOW, &mut rm),
        2.2544500000000003,
    );
}

#[test]
fn sma_rm_nolink_2() {
    let vec = common::PRICES;
    let mut rm = HashMap::from([
        ("src", rm::g_rm_sma(
            vec.iter(),
            &vec.len(),
            &common::WINDOW,
        )["src"].iter().map(|v| **v).collect::<Vec<f64>>(),)
    ]);
    assert_eq!(
        sma_rm::<f64, f64>(2.2547, &common::WINDOW, &mut rm),
        2.2544500000000003,
    );
}