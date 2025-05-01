use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use tokio::runtime::Runtime;
use reqwest::Client;
use bc_utils_lg::statics::settings::SETTINGS;

use bc_exch_api_funcs::bybit::account::wallet_balance::*;


fn wallet_balance_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("wallet_balance_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| wallet_balance_req(
            &cl,
            &SETTINGS.exch.api_key,
            &SETTINGS.exch.api_secret,
            &SETTINGS.exch.api_url,
            "UNIFIED",
            "USDT",
        ));
    });
}

fn wallet_balance_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("wallet_balance_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| wallet_balance_a(
            &cl,
            &SETTINGS.exch.api_key,
            &SETTINGS.exch.api_secret,
            &SETTINGS.exch.api_url,
            "UNIFIED",
            "USDT",
        ));
    });
}

criterion_group!(
    benches, 
    wallet_balance_req_lch_1,
    wallet_balance_a_lch_1,
);
criterion_main!(benches);