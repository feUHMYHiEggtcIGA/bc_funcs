use bc_utils_lg::statics::prices::SRC_VEC;

use bc_signals::train::mm::*;

#[test]
fn mm_coll_res_1(){
    // test it thoroughly
    assert_eq!(
        vec![-1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        mm_coll::<Vec<f64>>(
            &SRC_VEC[..10], 
            "open",
            &2, 
            &3,
            &0.0001,
            &0.01,
        ),
    );

}
