use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::market::klines::*;

fn klines_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("klines_lch_1", |b| {
        b.to_async(&rtm).iter(|| klines(
            "https://api.bybit.com", 
            "linear",
            "SUIUSDT",
            "1",
            &10,
            &0,
            &0,
        ));
    });
}

fn klines_aon_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("klines_aon_lch_1", |b| {
        b.to_async(&rtm).iter(|| klines_aon(
            "https://api.bybit.com", 
            "linear",
            "SUIUSDT",
            "1",
            &10,
            &0,
            &0,
        ));
    });
}

fn klines_symbols_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("klines_symbols_lch_1", |b| {
        b.to_async(&rtm).iter(|| klines_symbols(
            "https://api.bybit.com", 
            "linear",
            symbols.as_slice(),
            "1",
            &10,
            &0,
            &0,
        ));
    });
}

criterion_group!(
    benches, 
    klines_lch_1, 
    klines_aon_lch_1,
    klines_symbols_lch_1, 
);
criterion_main!(benches);