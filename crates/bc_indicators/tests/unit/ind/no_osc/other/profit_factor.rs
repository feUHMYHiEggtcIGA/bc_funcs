use bc_indicators::ind::no_osc::other::profit_factor::*;


#[test]
fn profit_factor_res_1() {
    assert_eq!(profit_factor(&[1.0, 2.0, -1.0]),3.0,)
}

#[test]
fn profit_factor_coll_res_1() {
    assert_eq!(
        profit_factor_coll::<Vec<f64>, f64, f64>(
            &[
                &[1.0, 2.0, -1.0],
                &[1.0, 2.0, -1.0],
            ]
        ),
        vec![3.0, 3.0],
    )
}