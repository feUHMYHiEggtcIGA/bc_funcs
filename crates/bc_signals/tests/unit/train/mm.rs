use bc_utils_lg::statics::prices::SRC_VEC;

use bc_signals::train::mm::*;

#[test]
fn mm_coll_res_1(){
    // test it thoroughly
    assert_eq!(
        (vec![-1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 
        vec![0.0, 0.0, 1.0, 0.0, -1.0, -1.0, 0.0, 0.0, 0.0, 0.0]),
        mm_coll(
            &SRC_VEC[..10], 
            "open",
            &3, 
            &2, 
            &0.001,
            &0.01,
        ),
    );
}