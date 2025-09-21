use reqwest::Client;
use bc_utils_lg::statics::settings::SETTINGS;

use bc_exch_api_funcs::bybit::account::acc_info::*;


#[tokio::test]
async fn acc_info_req_lch_1() {
    println!("{:#?}", acc_info_req(
        &Client::new(), 
        &SETTINGS.exch.api_key,
        &SETTINGS.exch.api_secret,
        &SETTINGS.exch.api_url,
    ).await.unwrap());
}

#[tokio::test]
async fn acc_info_a_lch_1() {
    println!("{:#?}", acc_info_a(
        &Client::new(), 
        &SETTINGS.exch.api_key,
        &SETTINGS.exch.api_secret,
        &SETTINGS.exch.api_url,
        &f64::INFINITY,
    ).await.unwrap());
}
