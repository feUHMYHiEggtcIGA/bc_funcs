use bc_indicators::indicators::no_oscillators::other::*;

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
