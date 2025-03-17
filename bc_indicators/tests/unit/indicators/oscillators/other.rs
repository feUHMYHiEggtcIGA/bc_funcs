use bc_utils::transf;

use super::super::super::super::common;
use bc_indicators::indicators::oscillators::other::*;
use bc_indicators::rm;

#[test]
fn t_rsi_rm_1() {
    let vec = common::PRICES;
    let (mut rm, mut rm_rma1, mut rm_rma2) = rm::g_rm_rsi(
        vec.iter(), 
        &vec.len(), 
        &common::WINDOW,
    );

    assert_eq!(
        transf::g_round_float(
            g_rsi_rm(
                &2.2547, 
                &mut rm, 
                &mut rm_rma1, 
                &mut rm_rma2,
            ), 
        4),
        40.4107,
    )
}

#[test]
fn g_rsi_float_1() {
    let vec = common::PRICES;

    assert_eq!(
        transf::g_round_float(
            g_rsi_float(vec.iter(), &vec.len(), &2,),
        4),
        40.4107,
    )
}