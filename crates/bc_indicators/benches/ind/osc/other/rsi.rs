use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bc_indicators::ind::osc::other::rsi::*;
use bc_indicators::bf::rsi::bf_rsi;
use bc_utils_lg::statics::prices::OPEN;
use bc_utils_lg::statics::settings::WINDOW;

fn rsi_bf_1(c: &mut Criterion) {
    let (
        mut bf,
        mut bf_rma1,
        mut bf_rma2,
    ) = bf_rsi(OPEN.as_slice(), &WINDOW, &true,);
    let price_last = OPEN.last().unwrap();
    c.bench_function("rsi_bf_1", |v| {
        v.iter(|| rsi_bf(
            black_box(price_last),
            &mut bf,
            &mut bf_rma1,
            &mut bf_rma2,
        ))
    });
}

fn rsi_f_1(c: &mut Criterion) {
    c.bench_function("rsi_f_1", |v| {
        v.iter(|| 
            rsi_f(
                black_box(OPEN.as_slice()),
                black_box(&WINDOW),
            )
        )
    });
}

criterion_group!(benches, rsi_bf_1, rsi_f_1);
criterion_main!(benches);