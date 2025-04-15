use bc_utils_lg::statics::prices::OPEN;
use bc_utils_lg::statics::settings::WINDOW;
use bc_indicators::ind::no_osc::trend::sma::*;
use bc_indicators::bf::sma::bf_sma;


#[test]
fn sma_bf_res_1() {
    let mut bf = bf_sma(
        OPEN.as_slice(),
        &WINDOW,
        &true,
    );
    assert_eq!(
        sma_bf::<f64>(2.2547, &WINDOW, &mut bf),
        2.2544500000000003,
    );
}

#[test]
fn sma_bf_res_skip_1() {
    let mut bf = bf_sma(
        &OPEN[2..],
        &WINDOW,
        &true,
    );
    assert_eq!(
        sma_bf::<f64>(2.2547, &WINDOW, &mut bf),
        2.2544500000000003,
    );
}

#[test]
fn sma_f_res_1(){
    assert_eq!(
        sma_f(OPEN.as_slice(), &WINDOW),
        2.2544500000000003,
    )
}

#[test]
fn sma_f_res_skip_1(){
    assert_eq!(
        sma_f(&OPEN[2..], &WINDOW),
        2.2544500000000003,
    )
}