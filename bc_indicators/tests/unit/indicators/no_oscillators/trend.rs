use bc_utils::transf;

use super::super::super::super::common;
use bc_indicators::indicators::no_oscillators::trend::*;
use bc_indicators::rm;


#[test]
fn t_alpha_ema_1() {
    assert_eq!(g_alpha_ema(&9.0), 0.2);
}

#[test]
fn t_alpha_rma_1() {
    assert_eq!(
        g_alpha_rma(&10.0),
        0.1,
    )
}

#[test]
fn t_sma_rm_1() {
    let vec = common::g_vec_prices();
    let mut rm = rm::g_rm_sma(
        vec.iter(),
        &vec.len(),
        &common::WINDOW,
    );

    assert_eq!(
        transf::g_round_float(g_sma_rm(&2.2547, &common::WINDOW, &mut rm), 4),
        2.2545,
    );
}

#[test]
fn t_ema_rm_1() {
    let vec = common::g_vec_prices();
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
fn t_rma_rm_1() {
    let vec = common::g_vec_prices();
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
