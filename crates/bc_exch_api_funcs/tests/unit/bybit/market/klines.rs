use bc_exch_api_funcs::bybit::market::klines::*;

#[tokio::test]
async fn klines_lch_1() -> Result<(), reqwest::Error>{
    klines(
        "https://api.bybit.com", 
        "linear",
        "SUIUSDT",
        "1",
        &10,
        &0,
        &0,
    )
        .await?;
    Ok(())
}

#[tokio::test]
async fn klines_symbols_lch_1() -> Result<(), reqwest::Error>{
    let symbols = vec!["SUIUSDT".to_string(), "ETHUSDT".to_string(), "ATOMUSDT".to_string()];
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
    Ok(())
}