use bc_indicators::ind::osc::mult::diff::*;


#[test]
fn mult_diff_res_1() {
    assert_eq!(mult_diff(&95.0, &100.0, &2.0), 0.10526315789473673)
}

#[test]
fn mult_diff_coll_res_1() {
    assert_eq!(
        mult_diff_coll::<Vec<f64>, _, _>(&[95.0, 95.0], &[100.0, 100.0], &2.0),
        vec![0.10526315789473673, 0.10526315789473673,],
    ) 
}