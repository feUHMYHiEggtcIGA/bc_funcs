use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bc_indicators::indicators::no_oscillators::trend::*;
use bc_indicators::rm::*;
use bc_indicators::common::{OPEN, WINDOW};

fn ema_rm_1(m: &mut Criterion) {
    let mut rm = rm_ema(OPEN.as_slice(), &WINDOW);
    let price_last = OPEN.last().unwrap();
    m.bench_function("ema_rm_1", |f| f.iter(
        || ema_rm(
            black_box(price_last),
            black_box(&mut rm),
        )
    ));
}

criterion_group!(benches, ema_rm_1);
criterion_main!(benches);