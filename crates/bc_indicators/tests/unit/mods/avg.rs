use bc_utils_lg::statics::prices::{OPEN, CLOSE};
use bc_utils::other::coll1_roll_replace_el;

use bc_indicators::mods::avg::*;


#[test]
fn avg_with_coll_res_1(){
    let src = avg_coll::<Vec<f64>, _>(
        OPEN.as_slice(), 
        &[coll1_roll_replace_el::<Vec<f64>, _, _,>(
            CLOSE.clone().as_mut_slice(),
            &1,
            f64::NAN,
        ).as_slice()],
    );
    assert_eq!(
        src.last().unwrap(),
        &2.25385,
    );
}