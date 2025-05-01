use reqwest::Client;
use bc_utils_lg::statics::settings::SETTINGS;

use bc_exch_api_funcs::bybit::account::wallet_balance::*;


#[tokio::test]
async fn wallet_balance_req_lch_1() {
    println!("{:#?}", wallet_balance_req(
        &Client::new(),
        &SETTINGS.exch.api_key,
        &SETTINGS.exch.api_secret,
        &SETTINGS.exch.api_url,
        "UNIFIED", 
        "USDT",
    ).await);
}

#[tokio::test]
async fn wallet_balance_a_lch_1() {
    println!("{:#?}", wallet_balance_a(
        &Client::new(),
        &SETTINGS.exch.api_key,
        &SETTINGS.exch.api_secret,
        &SETTINGS.exch.api_url,
        "UNIFIED", 
        "USDT",
    ).await);
}