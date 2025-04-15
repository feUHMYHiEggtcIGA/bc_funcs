use bc_indicators::ind::no_osc::other::percent::*;


#[test]
fn percent_res_1() {
    assert_eq!(percent::<f64, &f64>(&105.0, &100.0,),0.05);
}

#[test]
fn percent_coll_res_1() {
    assert_eq!(
        percent_coll::<Vec<f64>, f64, f64>(&[105.0, 105.0], &[100.0, 100.0],), 
        vec![0.05, 0.05],
    )
}