use super::super::super::super::common;

use bc_indicators::indicators::oscillators::trend::*;
use bc_indicators::rm;

#[test]
fn tqo_float_1() {
    let src = common::PRICES;

    assert_eq!(
        tqo_b_float(
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
fn tqo_float_2() {
    let src = common::PRICES;
    let iter_ = src.iter().skip(1);

    assert_eq!(
        tqo_b_float(
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
fn tqo_float_3() {
    let src = common::PRICES;
    let iter_ = src.iter();

    assert_eq!(
        tqo_b_float(
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
fn tqo_float_4() {
    let src = common::PRICES;
    let iter_ = src.iter().skip(1);

    assert_eq!(
        tqo_b_float(
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
fn tqo_rm_1() {
    let src = common::PRICES;
    let iter_ = src.iter().skip(1);
    let count = iter_.clone().count();
    let (
        mut rm_, 
        mut rm_fast, 
        mut rm_slow, 
        mut rm_sma
    ) = rm::g_rm_tqo(
        iter_.clone(),
        &count, 
        &2, 
        &3, 
        &4, 
        &2,
        &10,
        "linear"
    );
    assert_eq!(
        tqo_b_rm(
            iter_.last().unwrap(), 
            &2, 
            &2.0, 
            "linear", 
            &mut rm_, 
            &mut rm_fast, 
            &mut rm_slow, 
            &mut rm_sma
        ),
        0.8205694494455186
    );
}

#[test]
fn tqo_rm_2() {
    let src = common::PRICES;
    let iter_ = src.iter().skip(1);
    let count = iter_.clone().count();
    let (
        mut rm_, 
        mut rm_fast, 
        mut rm_slow, 
        mut rm_sma
    ) = rm::g_rm_tqo(
        iter_.clone(),
        &count, 
        &2, 
        &3, 
        &4, 
        &2,
        &10,
        "squared"
    );

    assert_eq!(
        tqo_b_rm(
            iter_.last().unwrap(), 
            &2, 
            &2.0, 
            "squared", 
            &mut rm_, 
            &mut rm_fast, 
            &mut rm_slow, 
            &mut rm_sma
        ),
        0.8044721930064004
    );
}