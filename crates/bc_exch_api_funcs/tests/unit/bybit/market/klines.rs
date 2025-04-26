use bc_exch_api_funcs::bybit::market::klines::*;

#[tokio::test]
async fn klines_lch_1() {
    klines(
        "https://api.bybit.com", 
        "linear",
        "SUIUSDT",
        "1",
        &10,
        &0,
        &0,
    )
        .await
        .unwrap();
}

#[tokio::test]
async fn klines_a_lch_1() {
    klines_a(
        "https://api.bybit.com", 
        "linear",
        "SUIUSDT",
        "1",
        &10,
        &0,
        &0,
    )
        .await;
}

#[tokio::test]
async fn kline_symbols_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(), 
        "ETHUSDT".to_string(), 
        "ATOMUSDT".to_string(),
    ];
    kline_symbols(
        "https://api.bybit.com", 
        "linear",
        symbols.as_slice(),
        "1",
    )
        .await;
}

#[tokio::test]
async fn kline_symbols_a_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(), 
        "ETHUSDT".to_string(), 
        "ATOMUSDT".to_string(),
    ];
    kline_symbols_a(
        "https://api.bybit.com", 
        "linear",
        symbols.as_slice(),
        "1",
    )
        .await;
}

#[tokio::test]
async fn kline_symbols_ao_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(), 
        "ETHUSDT".to_string(), 
        "ATOMUSDT".to_string(),
    ];
    kline_symbols_ao(
        "https://api.bybit.com", 
        "linear",
        symbols.as_slice(),
        "1",
    )
        .await;
}

#[tokio::test]
async fn klines_symbols_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(), 
        "ETHUSDT".to_string(), 
        "ATOMUSDT".to_string(),
    ];
    klines_symbols(
        "https://api.bybit.com", 
        "linear",
        symbols.as_slice(),
        "1",
        &10,
        &0,
        &0,
    )
        .await;
}

#[tokio::test]
async fn klines_symbols_a_lch_1() {
    let symbols = vec![
        "SUIUSDT".to_string(), 
        "ETHUSDT".to_string(), 
        "ATOMUSDT".to_string(),
    ];
    klines_symbols_a(
        "https://api.bybit.com", 
        "linear",
        symbols.as_slice(),
        "1",
        &10,
        &0,
        &0,
    )
        .await;
}