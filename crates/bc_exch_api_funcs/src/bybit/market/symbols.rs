use bc_utils_lg::structs::exch::bybit::result::RESULT_EXCH_BYBIT;
use bc_utils_lg::structs::exch::bybit::symbols::{
    RESULT_SYMBOLS, 
    RESULT_SYMBOLS1,
};
use reqwest::{get, Error as Error_req};
use bc_core_funcs::mechanisms::all_or_nothing;

use crate::bybit::url_const::TICKERS;


pub async fn symbols_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    base_coin: &str,
    exp_date: &str
) -> Result<RESULT_EXCH_BYBIT<RESULT_SYMBOLS>, Error_req>
{
    get(
        format!(
            "{api_url}{TICKERS}\
            ?category={category}\
            &symbol={symbol}\
            &baseCoin={base_coin}\
            &expDate={exp_date}"
        )
    )
        .await?
        .json::<RESULT_EXCH_BYBIT<RESULT_SYMBOLS>>()
    .await
}

pub async fn symbols(
    api_url: &str,
    category: &str,
    symbol: &str,
    base_coin: &str,
    exp_date: &str
) -> Result<Vec<RESULT_SYMBOLS1>, Box<dyn std::error::Error>>
{
    Ok(symbols_req(
        api_url, 
        category, 
        symbol, 
        base_coin, 
        exp_date
    ).await?.result.list)
}

pub async fn symbols_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    base_coin: &str,
    exp_date: &str
) -> Vec<RESULT_SYMBOLS1>
{
    all_or_nothing(async || symbols(
        api_url, 
        category, 
        symbol, 
        base_coin, 
        exp_date
    ).await).await
}