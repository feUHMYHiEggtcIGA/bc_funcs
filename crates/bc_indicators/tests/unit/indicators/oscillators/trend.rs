use bc_utils_lg::statics::prices::OPEN;

use bc_indicators::indicators::oscillators::trend::*;
use bc_indicators::rm;

#[test]
fn tqo_float_1() {
    assert_eq!(
        tqo_b_float(
            OPEN.as_slice(),
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
    assert_eq!(
        tqo_b_float(
            OPEN.as_slice(),
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
fn tqo_float_skip_1() {
    assert_eq!(
        tqo_b_float(
            &OPEN[2..],
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
fn tqo_float_skip_2() {
    assert_eq!(
        tqo_b_float(
            &OPEN[2..],
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
    let (
        mut rm_, 
        mut rm_fast, 
        mut rm_slow, 
        mut rm_sma
    ) = rm::rm_tqo(
        OPEN.as_slice(),
        &2, 
        &3, 
        &4, 
        &2,
        &10,
        "linear"
    );
    assert_eq!(
        tqo_b_rm(
            OPEN.last().unwrap(),
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
    let (
        mut rm_, 
        mut rm_fast, 
        mut rm_slow, 
        mut rm_sma
    ) = rm::rm_tqo(
        OPEN.as_slice(),
        &2, 
        &3, 
        &4, 
        &2,
        &10,
        "squared"
    );
    assert_eq!(
        tqo_b_rm(
            OPEN.last().unwrap(),
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

#[test]
fn tqo_rm_skip_1() {
    let (
        mut rm_, 
        mut rm_fast, 
        mut rm_slow, 
        mut rm_sma
    ) = rm::rm_tqo(
        &OPEN[2..],
        &2, 
        &3, 
        &4, 
        &2,
        &10,
        "linear"
    );
    assert_eq!(
        tqo_b_rm(
            OPEN.last().unwrap(),
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
fn tqo_rm_skip_2() {
    let (
        mut rm_, 
        mut rm_fast, 
        mut rm_slow, 
        mut rm_sma
    ) = rm::rm_tqo(
        &OPEN[2..],
        &2, 
        &3, 
        &4, 
        &2,
        &10,
        "squared"
    );
    assert_eq!(
        tqo_b_rm(
            OPEN.last().unwrap(),
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