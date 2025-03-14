use bc_utils::transf;

// mod common;
use super::super::common;
use bc_indicators::no_abstract::indicators::no_oscillators::other::*;
use bc_indicators::no_abstract::indicators::oscillators::other::*;
use bc_indicators::no_abstract::rm;


#[test]
fn t_percent_1() {
    assert_eq!(
        g_percent(&100.0, &105.0),
        0.05
    );
}

#[test]
fn t_profit_factor_1() {
    assert_eq!(
        g_profit_factor(vec![1.0, 2.0, -1.0].iter()),
        3.0,
    )
}

#[test]
fn g_rsi_rm_1() {
    let vec = common::g_vec_prices();
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
        40.4103,
    )
}

#[test]
fn g_rsi_float_1() {
    let vec = vec![
        2.2599,
        2.2654, 2.2742, 2.2736, 2.2706, 2.2736, 
        2.2735, 2.2733, 2.2624, 2.2618, 2.2628, 
        2.2649, 2.2591, 2.2577, 2.2546, 2.2584, 
        2.2555, 2.2553, 2.2559, 2.2542, 2.2547,
    ];

    assert_eq!(
        transf::g_round_float(
            g_rsi_float(vec.iter(), &2,),
        4),
        40.4103,
    )
}