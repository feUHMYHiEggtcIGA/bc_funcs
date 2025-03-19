use bc_indicators::indicators::oscillators::multipliers::*;


#[test]
fn mult_rsi() {
    assert_eq!(
        g_mult_rsi(&85.0, &30.0, &15.0, &100.0),
        0.5
    )
}

#[test]
fn mult_diff_v() {
    assert_eq!(
        g_mult_diff_v(&2.0, &2.5, &2.0),
        0.5
    )
}