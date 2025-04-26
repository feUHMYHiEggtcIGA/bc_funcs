use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::market::symbols::*;

fn symbols_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("symbols_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| symbols_req(
            "https://api.bybit.com", 
            "linear",
            "",
            "",
            "",
        ));
    });
}

fn symbols_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("symbols_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| symbols_a(
            "https://api.bybit.com", 
            "linear",
            "",
            "",
            "",
        ));
    });
}

criterion_group!(
    benches, 
    symbols_req_lch_1,
    symbols_a_lch_1,
);
criterion_main!(benches);