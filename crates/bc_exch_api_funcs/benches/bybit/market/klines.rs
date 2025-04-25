use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use tokio::runtime::Runtime;
use bc_utils_lg::enums::indicators::T_ARGS;

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

fn klines_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("klines_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| klines_a(
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

fn kline_symbols_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("kline_symbols_lch_1", |b| {
        b.to_async(&rtm).iter(|| kline_symbols(
            "https://api.bybit.com", 
            "linear",
            symbols.as_slice(),
            "1",
        ));
    });
}

fn kline_symbols_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("kline_symbols_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| kline_symbols_a(
            "https://api.bybit.com", 
            "linear",
            symbols.as_slice(),
            "1",
        ));
    });
}

fn kline_symbols_ao_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("kline_symbols_ao_lch_1", |b| {
        b.to_async(&rtm).iter(|| kline_symbols_ao(
            "https://api.bybit.com", 
            "linear",
            symbols.as_slice(),
            "1",
        ));
    });
}

fn kline_symbols_ao_abstr_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    let args = vec![
        T_ARGS::Str("https://api.bybit.com"),
        T_ARGS::Str("linear"),
        T_ARGS::Slice(symbols.as_slice()),
        T_ARGS::Str("1"),
    ];
    c.bench_function("kline_symbols_ao_abstr_lch_1", |b| {
        b.to_async(&rtm).iter(|| kline_symbols_ao_abstr(
            &args,
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

fn klines_symbols_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
    c.bench_function("klines_symbols_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| klines_symbols_a(
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
    klines_a_lch_1,
    kline_symbols_lch_1,
    kline_symbols_a_lch_1,
    kline_symbols_ao_lch_1,
    kline_symbols_ao_abstr_lch_1,
    klines_symbols_lch_1, 
    klines_symbols_a_lch_1,
);
criterion_main!(benches);