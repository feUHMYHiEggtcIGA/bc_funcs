use super::super::super::super::super::common;


use bc_indicators::no_abstract::indicators::oscillators::trend::*;

#[test]
fn t_tqo_rm() {
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