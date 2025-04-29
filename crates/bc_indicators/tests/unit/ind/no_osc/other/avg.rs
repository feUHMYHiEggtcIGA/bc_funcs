use bc_utils_lg::statics::prices::{
    OPEN, 
    HIGH, 
    LOW,
};
use bc_utils::nums::avg as avg_f;

use bc_indicators::ind::no_osc::other::avg::*;


#[test]
fn avg_coll_res_1(){
    assert_eq!(
        avg_coll::<Vec<f64>, _,>(
            &[
                OPEN.as_slice(), 
                HIGH.as_slice(), 
                LOW.as_slice()
            ],
        ).last().unwrap(),
        &avg_f::<f64, &f64>(&[
            OPEN.last().unwrap(), 
            HIGH.last().unwrap(), 
            LOW.last().unwrap()
        ]),
    )
}

#[test]
fn avg_coll_res_skip_1(){
    assert_eq!(
        avg_coll::<Vec<f64>, _,>(
            &[
                OPEN.as_slice(), 
                &HIGH[1..], 
                LOW.as_slice()
            ],
        ).last().unwrap(),
        &avg_f::<f64, &f64>(&[
            OPEN.last().unwrap(), 
            HIGH.last().unwrap(), 
            LOW.last().unwrap()
        ]),
    )
}