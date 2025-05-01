use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use tokio::runtime::Runtime;
use reqwest::Client;
use bc_utils_lg::statics::settings::SETTINGS;

use bc_exch_api_funcs::bybit::account::acc_info::*;

fn acc_info_req_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("acc_info_req_lch_1", |b| {
        b.to_async(&rtm).iter(|| acc_info_req(
            &cl,
            &SETTINGS.exch.api_key,
            &SETTINGS.exch.api_secret,
            &SETTINGS.exch.api_url,
        ));
    });
}

fn acc_info_a_lch_1(c: &mut Criterion) {
    let rtm = Runtime::new().unwrap();
    let cl = Client::new();
    c.bench_function("acc_info_a_lch_1", |b| {
        b.to_async(&rtm).iter(|| acc_info_a(
            &cl,
            &SETTINGS.exch.api_key,
            &SETTINGS.exch.api_secret,
            &SETTINGS.exch.api_url,
        ));
    });
}

criterion_group!(
    benches, 
    acc_info_req_lch_1,
    acc_info_a_lch_1,
);
criterion_main!(benches);