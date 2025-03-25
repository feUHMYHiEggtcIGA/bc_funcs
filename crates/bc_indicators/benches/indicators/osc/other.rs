use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bc_indicators::indicators::oscillators::other::*;
use bc_indicators::rm::*;
use bc_indicators::common;

fn rsi_rm_1(c: &mut Criterion) {
    let (
        mut rm,
        mut rm_rma1,
        mut rm_rma2,
    ) = rm_rsi(common::OPEN.as_slice(), &common::WINDOW);
    let price_last = common::OPEN.last().unwrap();
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
                black_box(common::OPEN.as_slice()),
                black_box(&common::WINDOW),
            )
        )
    });
}

criterion_group!(benches, rsi_rm_1, rsi_float_1);
criterion_main!(benches);