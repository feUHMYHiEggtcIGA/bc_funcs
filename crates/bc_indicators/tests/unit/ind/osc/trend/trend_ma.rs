use bc_indicators::ind::osc::trend::trend_ma::*;


#[test]
fn trend_ma_coll_res_1() {
    assert_eq!(
        vec![1.0, 0.0, 0.0],
        trend_ma_coll::<Vec<f64>, _>(&[1.0, 1.1, 1.2, 0.9, 0.9])
    );

}