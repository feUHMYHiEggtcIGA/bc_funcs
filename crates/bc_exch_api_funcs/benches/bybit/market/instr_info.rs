use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use tokio::runtime::Runtime;

use bc_exch_api_funcs::bybit::market::instr_info::*;


fn instr_info_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("instr_info_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| instr_info_req(
            "https://api.bybit.com",
            "linear", 
            "BTCUSDT",
            "",
            "",
            &1,
            "",
        ));
    });
}

fn instr_info_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("instr_info_lch_1", |b| {
        b.to_async(&rtm).iter(|| instr_info(
            "https://api.bybit.com",
            "linear", 
            "BTCUSDT",
            "",
            "",
        ));
    });
}

fn instr_info_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    c.bench_function("instr_info_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| instr_info_a(
            "https://api.bybit.com",
            "linear", 
            "BTCUSDT",
            "",
            "",
        ));
    });
}

fn instrs_info_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = &["SUIUSDT".to_string(), "UNIUSDT".to_string(), "ETHUSDT".to_string()];
    c.bench_function("instrs_info_lch_1", |b| {
        b.to_async(&rtm).iter(|| instrs_info(
            "https://api.bybit.com",
            "linear", 
            symbols,
            "",
            "",
        ));
    });
}

fn instrs_info_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let symbols = &["SUIUSDT".to_string(), "UNIUSDT".to_string(), "ETHUSDT".to_string()];
    c.bench_function("instrs_info_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| instrs_info_a(
            "https://api.bybit.com",
            "linear", 
            symbols,
            "",
            "",
        ));
    });
}

criterion_group!(
    benches, 
    instr_info_req_lch_1,
    instr_info_lch_1,
    instr_info_a_lch_1,
    instrs_info_lch_1,
    instrs_info_a_lch_1,
);
criterion_main!(benches);