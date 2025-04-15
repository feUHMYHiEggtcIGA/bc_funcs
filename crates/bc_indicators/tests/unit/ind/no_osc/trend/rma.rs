use bc_utils_lg::statics::prices::OPEN;
use bc_utils_lg::statics::settings::WINDOW;
use bc_indicators::ind::no_osc::trend::rma::*;
use bc_indicators::bf::rma::bf_rma;


#[test]
fn rma_bf_res_1() {
    let mut bf = bf_rma(
        OPEN.as_slice(),
        &WINDOW,
        &true,
    );

    assert_eq!(
        rma_bf(&2.2547, &mut bf),
        2.2548879972457887,
    );
}

#[test]
fn rma_bf_res_skip_1() {
    let mut bf = bf_rma(
        &OPEN[2..],
        &WINDOW,
        &true,
    );

    assert_eq!(
        rma_bf(&2.2547, &mut bf),
        2.2548879972457887,
    );
}

#[test]
fn rma_f_res_1() {
    assert_eq!(
        rma_f(OPEN.as_slice(), &WINDOW),
        2.2548879972457887,
    );
}

#[test]
fn rma_f_res_skip_1() {
    assert_eq!(
        rma_f(&OPEN[2..], &WINDOW),
        2.2548879972457887,
    );
}