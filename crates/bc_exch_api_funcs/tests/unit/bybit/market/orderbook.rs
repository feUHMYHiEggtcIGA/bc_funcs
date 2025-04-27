use tokio;

use bc_exch_api_funcs::bybit::market::orderbook::*;


#[tokio::test]
async fn orderbook_req_lch_1(){
    println!("{:#?}", orderbook_req(
        "https://api.bybit.com",
        "linear", 
        "SUIUSDT",
        &10,
    )
        .await
        .unwrap());
}

#[tokio::test]
async fn orderbook_a_lch_1(){
    orderbook_a(
        "https://api.bybit.com",
        "linear", 
        "SUIUSDT",
        &10,
    )
        .await;
}

#[tokio::test]
async fn orderbooks_lch_1(){
    let symbols = &[
        "SUIUSDT".to_string(), 
        "WALRUSUSDT".to_string(), 
        "ATOMUSDT".to_string(),
    ];
    orderbooks(
        "https://api.bybit.com",
        "linear", 
        symbols,
        &10,
    )
        .await;
}

// #[tokio::test]
// async fn orderbooks_a_lch_1(){
//     let symbols = &[
//         "SUIUSDT".to_string(), 
//         "WALRUSUSDT".to_string(), 
//         "ATOMUSDT".to_string(),
//     ];
//     orderbooks_a(
//         "https://api.bybit.com",
//         "linear", 
//         symbols,
//         &10,
//     )
//         .await;
// }