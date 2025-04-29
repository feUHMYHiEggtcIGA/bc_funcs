use bc_utils_lg::statics::settings::SETTINGS_IND_TEST;
use bc_utils_lg::statics::prices::{
    OPEN,
    HIGH,
    LOW,
    SRCS
};

use bc_indicators::map::ind::map_ind_coll;
use bc_indicators::map::args::map_args_ind;
use bc_indicators::gw::src::*;
use bc_indicators::ind::no_osc::other::avg::avg_coll;


#[test]
fn src_from_settings_res_1(){
    let map_ind_coll_ = map_ind_coll();
    let map_args_ = map_args_ind(&SETTINGS_IND_TEST);
    assert_eq!(
        avg_coll::<Vec<f64>, _>(&[
            OPEN.as_slice(),
            &HIGH[..HIGH.len() - 1],
            &LOW[..LOW.len() - 1],
        ]),
        avg_coll::<Vec<f64>, _,>(
            src_from_settings(
                &SETTINGS_IND_TEST["src_1"].used_ind, 
                &SETTINGS_IND_TEST["src_1"].used_src, 
                &SETTINGS_IND_TEST, 
                &SRCS, 
                &map_ind_coll_, 
                &map_args_,
            )
                .iter()
                .map(Vec::as_slice)
                .collect::<Vec<&[f64]>>()
                .as_slice()
        )
    )
}