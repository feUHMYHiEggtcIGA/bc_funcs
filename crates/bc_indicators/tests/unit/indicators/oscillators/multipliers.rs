use bc_indicators::indicators::oscillators::multipliers::*;


#[test]
fn mult_rsi_1() {
    assert_eq!(mult_rsi(&85.0, &30.0, &15.0, &100.0), 0.5) 
}

#[test]
fn mult_diff_v_1() {
    assert_eq!(mult_diff_v(&2.0, &2.5, &2.0),0.5) 
}