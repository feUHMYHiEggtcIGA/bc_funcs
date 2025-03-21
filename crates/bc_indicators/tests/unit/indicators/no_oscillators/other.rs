use bc_indicators::indicators::no_oscillators::other::*;

#[test]
fn percent_1() {
    assert_eq!(g_percent::<f64, &f64>(&100.0, &105.0),0.05);
}

#[test]
fn percent_2() {
    assert_eq!(g_percent::<f64, f64>(100.0, 105.0),0.05);
}

#[test]
fn profit_factor_1() {
    assert_eq!(g_profit_factor(vec![1.0, 2.0, -1.0].iter()),3.0,)
}
