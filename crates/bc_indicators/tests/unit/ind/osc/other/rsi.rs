use bc_utils_lg::statics::prices::*;
use bc_utils_lg::statics::settings::*;
use bc_utils::other::coll1_roll_replace_el;

use bc_indicators::ind::osc::other::rsi::*;
use bc_indicators::bf::rsi::bf_rsi;

#[test]
fn rsi_bf_res_1() {
    let (
        mut bf, 
        mut bf_rma1, 
        mut bf_rma2
    ) = bf_rsi(
        OPEN.as_slice(),
        &WINDOW,
        &true,
    );

    assert_eq!(
        rsi_bf(
            &2.2547, 
            &mut bf, 
            &mut bf_rma1, 
            &mut bf_rma2,
        ), 
        40.410730678054115,
    )
}

// #[test]
// fn rsi_bf_res_avg_1() {
//     let src = avg_coll::<Vec<f64>, _>(
//         OPEN.as_slice(), 
//         &[coll1_roll_replace_el::<Vec<f64>, _, _,>(
//             CLOSE.clone().as_mut_slice(),
//             &1,
//             f64::NAN,
//         ).as_slice()],
//     );
//     let (
//         mut bf, 
//         mut bf_rma1, 
//         mut bf_rma2
//     ) = bf_rsi(
//         src.as_slice(),
//         &WINDOW,
//         &true,
//     );
    
//     assert_eq!(
//         rsi_bf(
//             src.last().unwrap(),
//             &mut bf, 
//             &mut bf_rma1,
//             &mut bf_rma2,
//         ), 
//         4.957159648988011,
//     )
// }

#[test]
fn rsi_bf_res_skip_1() {
    let (
        mut bf, 
        mut bf_rma1, 
        mut bf_rma2
    ) = bf_rsi(
        &OPEN[2..],
        &WINDOW,
        &true,
    );

    assert_eq!(
        rsi_bf(
            &2.2547, 
            &mut bf, 
            &mut bf_rma1, 
            &mut bf_rma2,
        ), 
        40.410730678054115,
    )
}

#[test]
fn rsi_f_res_1() {
    assert_eq!(
        rsi_f(OPEN.as_slice(),  &2,),
        40.410730678054115,
    )
}

#[test]
fn rsi_f_res_skip_1() {
    assert_eq!(
        rsi_f(&OPEN[2..],  &2,),
        40.410730678054115,
    )
}