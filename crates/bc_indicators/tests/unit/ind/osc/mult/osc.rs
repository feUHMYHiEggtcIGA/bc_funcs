use bc_indicators::ind::osc::mult::osc::*;


#[test]
fn mult_osc_res_1() {
    assert_eq!(mult_osc(&85.0, &30.0, &15.0, &100.0), 0.5) 
}

#[test]
fn mult_osc_coll_res_1() {
    assert_eq!(
        mult_osc_coll::<Vec<f64>, _, _>(&[85.0, 85.0], &30.0, &15.0, &100.0), 
        vec![0.5, 0.5,],
    ) 
}