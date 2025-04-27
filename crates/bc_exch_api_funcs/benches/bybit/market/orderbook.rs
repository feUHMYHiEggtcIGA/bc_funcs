use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::market::orderbook::*;


fn orderbook_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("orderbook_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| orderbook_req(
            "https://api.bybit.com",
            "linear", 
            "SUIUSDT",
            &10,
        ));
    });
}

fn orderbook_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("orderbook_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| orderbook_a(
            "https://api.bybit.com",
            "linear", 
            "SUIUSDT",
            &10,
        ));
    });
}

fn orderbooks_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = &[
        "SUIUSDT".to_string(), 
        "WALRUSUSDT".to_string(), 
        "ATOMUSDT".to_string(),
    ];
    c.bench_function("orderbooks_lch_1", |b| {
        b.to_async(&rtm).iter(|| orderbooks(
            "https://api.bybit.com",
            "linear", 
            symbols,
            &10,
        ));
    });
}

fn orderbooks_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = &[
        "SUIUSDT".to_string(), 
        "WALRUSUSDT".to_string(), 
        "ATOMUSDT".to_string(),
    ];
    c.bench_function("orderbooks_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| orderbooks_a(
            "https://api.bybit.com",
            "linear", 
            symbols,
            &10,
        ));
    });
}

criterion_group!(
    benches, 
    orderbook_req_lch_1,
    orderbook_a_lch_1,
    orderbooks_lch_1,
    orderbooks_a_lch_1,
);
criterion_main!(benches);