use super::super::super::super::super::common;

use bc_indicators::no_abstract::indicators::oscillators::trend::*;

#[test]
fn t_tqo_rm_1() {
    let src = common::g_vec_prices();

    assert_eq!(
        g_tqo_b_float(
            src.iter(), 
            &src.len(), 
            &2, 
            &3, 
            &4, 
            &2, 
            &2.0, 
            &10,
            "linear"
        ),
        0.8205694494455186
    );
}

#[test]
fn t_tqo_rm_2() {
    let src = common::g_vec_prices();
    let iter_ = src.iter().skip(1);

    assert_eq!(
        g_tqo_b_float(
            iter_.clone(),
            &iter_.count(),
            &2, 
            &3, 
            &4, 
            &2, 
            &2.0, 
            &10,
            "linear"
        ),
        0.8205694494455186
    );
}

#[test]
fn t_tqo_rm_3() {
    let src = common::g_vec_prices();
    let iter_ = src.iter();

    assert_eq!(
        g_tqo_b_float(
            iter_.clone(),
            &iter_.count(),
            &2, 
            &3, 
            &4, 
            &2, 
            &2.0, 
            &10,
            "squared"
        ),
        0.8044721930064004
    );
}

#[test]
fn t_tqo_rm_4() {
    let src = common::g_vec_prices();
    let iter_ = src.iter().skip(1);

    assert_eq!(
        g_tqo_b_float(
            iter_.clone(),
            &iter_.count(),
            &2, 
            &3, 
            &4, 
            &2, 
            &2.0, 
            &10,
            "squared"
        ),
        0.8044721930064004
    );
}