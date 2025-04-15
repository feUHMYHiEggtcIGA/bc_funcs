use bc_utils_lg::statics::prices::OPEN;

use bc_indicators::ind::osc::trend::tqo_b::*;
use bc_indicators::bf::tqo_b::bf_tqo_b;


#[test]
fn tqo_bf_res_1() {
    let (
        mut bf_, 
        mut bf_fast, 
        mut bf_slow, 
        mut bf_sma
    ) = bf_tqo_b(
        OPEN.as_slice(),
        &2, 
        &3, 
        &4, 
        &2,
        &10,
        "linear",
        &true,
    );
    assert_eq!(
        tqo_b_bf(
            OPEN.last().unwrap(),
            &2, 
            &2.0, 
            "linear", 
            &mut bf_, 
            &mut bf_fast, 
            &mut bf_slow, 
            &mut bf_sma
        ),
        0.8205694494455186
    );
}

#[test]
fn tqo_bf_res_2() {
    let (
        mut bf_, 
        mut bf_fast, 
        mut bf_slow, 
        mut bf_sma
    ) = bf_tqo_b(
        OPEN.as_slice(),
        &2, 
        &3, 
        &4, 
        &2,
        &10,
        "squared",
        &true,
    );
    assert_eq!(
        tqo_b_bf(
            OPEN.last().unwrap(),
            &2, 
            &2.0, 
            "squared", 
            &mut bf_, 
            &mut bf_fast, 
            &mut bf_slow, 
            &mut bf_sma
        ),
        0.8044721930064004
    );
}

#[test]
fn tqo_bf_res_skip_1() {
    let (
        mut bf_, 
        mut bf_fast, 
        mut bf_slow, 
        mut bf_sma
    ) = bf_tqo_b(
        &OPEN[2..],
        &2, 
        &3, 
        &4, 
        &2,
        &10,
        "linear",
        &true,
    );
    assert_eq!(
        tqo_b_bf(
            OPEN.last().unwrap(),
            &2, 
            &2.0, 
            "linear", 
            &mut bf_, 
            &mut bf_fast, 
            &mut bf_slow, 
            &mut bf_sma
        ),
        0.8205694494455186
    );
}

#[test]
fn tqo_bf_res_skip_2() {
    let (
        mut bf_, 
        mut bf_fast, 
        mut bf_slow, 
        mut bf_sma
    ) = bf_tqo_b(
        &OPEN[2..],
        &2, 
        &3, 
        &4, 
        &2,
        &10,
        "squared",
        &true,
    );
    assert_eq!(
        tqo_b_bf(
            OPEN.last().unwrap(),
            &2, 
            &2.0, 
            "squared", 
            &mut bf_, 
            &mut bf_fast, 
            &mut bf_slow, 
            &mut bf_sma
        ),
        0.8044721930064004
    );
}

#[test]
fn tqo_f_res_1() {
    assert_eq!(
        tqo_b_f(
            OPEN.as_slice(),
            &2, 
            &3, 
            &4, 
            &2, 
            &10,
            &2.0, 
            "linear"
        ),
        0.8205694494455186
    );
}

#[test]
fn tqo_f_res_2() {
    assert_eq!(
        tqo_b_f(
            OPEN.as_slice(),
            &2, 
            &3, 
            &4, 
            &2, 
            &10,
            &2.0, 
            "squared"
        ),
        0.8044721930064004
    );
}

#[test]
fn tqo_f_res_skip_1() {
    assert_eq!(
        tqo_b_f(
            &OPEN[2..],
            &2, 
            &3, 
            &4, 
            &2, 
            &10,
            &2.0, 
            "linear"
        ),
        0.8205694494455186
    );
}

#[test]
fn tqo_f_res_skip_2() {
    assert_eq!(
        tqo_b_f(
            &OPEN[2..],
            &2, 
            &3, 
            &4, 
            &2, 
            &10,
            &2.0, 
            "squared"
        ),
        0.8044721930064004
    );
}