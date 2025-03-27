use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bc_indicators::indicators::oscillators::other::*;
use bc_indicators::rm::*;
use bc_utils_lg::statics::prices::OPEN;
use bc_utils_lg::statics::settings::WINDOW;

fn rsi_rm_1(c: &mut Criterion) {
    let (
        mut rm,
        mut rm_rma1,
        mut rm_rma2,
    ) = rm_rsi(OPEN.as_slice(), &WINDOW);
    let price_last = OPEN.last().unwrap();
    c.bench_function("rsi_rm_1", |v| {
        v.iter(|| rsi_rm(
            black_box(price_last),
            &mut rm,
            &mut rm_rma1,
            &mut rm_rma2,
        ))
    });
}

fn rsi_float_1(c: &mut Criterion) {
    c.bench_function("rsi_float_1", |v| {
        v.iter(|| 
            rsi_float(
                black_box(OPEN.as_slice()),
                black_box(&WINDOW),
            )
        )
    });
}

criterion_group!(benches, rsi_rm_1, rsi_float_1);
criterion_main!(benches);