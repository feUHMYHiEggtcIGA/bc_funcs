use tokio;

use bc_exch_api_funcs::bybit::market::instr_info::*;


#[tokio::test]
async fn instr_unfo_lch_1(){
    instr_info(
        "https://api.bybit.com",
        "linear", 
        "SUIUSDT", 
        "",
        "",
        &1,
        "",
    )
        .await
        .unwrap();
}