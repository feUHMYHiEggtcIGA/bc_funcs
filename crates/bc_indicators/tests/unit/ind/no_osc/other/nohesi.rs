use bc_utils_lg::statics::prices::OPEN;

use bc_indicators::ind::no_osc::other::nohesi::*;


#[test]
fn nohesi_coll_res_1(){
    assert_eq!(
        nohesi_coll::<Vec<f64>, _,>(
            OPEN.as_slice(), 
            &0.00001,
        ).last().unwrap(),
        // this test is meaningless
        &nohesi_f(
            OPEN.as_slice(),
            &0.00001,
        ),
    )
}