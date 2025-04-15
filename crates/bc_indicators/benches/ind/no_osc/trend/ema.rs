use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bc_indicators::ind::no_osc::trend::ema::ema_bf;
use bc_indicators::bf::ema::bf_ema;
use bc_utils_lg::statics::prices::{OPEN, OPEN_LAST};
use bc_utils_lg::statics::settings::WINDOW;

fn ema_rm_1(m: &mut Criterion) {
    let mut rm = bf_ema(OPEN.as_slice(), &WINDOW, &true,);
    m.bench_function("ema_bf_1", |f| f.iter(
        || ema_bf(
            black_box(&OPEN_LAST),
            black_box(&mut rm),
        )
    ));
}

criterion_group!(benches, ema_rm_1);
criterion_main!(benches);