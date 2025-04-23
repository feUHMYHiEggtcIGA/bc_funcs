#![allow(non_camel_case_types)]

use reqwest::{
    Error as Error_req,
    get,
};
use bc_utils_lg::structs::exch::bybit::klines::RESULT_KLINE_W;
use futures::future::join_all;
use bc_utils_lg::types::maps_abstr::MAP;

use crate::bybit::api_url::KLINE;


pub async fn klines(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval: &str,
    limit: &u32,
    start: &u64,
    end: &u64,
) -> Result<RESULT_KLINE_W, Error_req>
{
    get(
        format!(
            "{api_url}{KLINE}\
            ?category={category}\
            &symbol={symbol}\
            &interval={interval}\
            &limit={limit}\
            &start={start}\
            &end={end}"
        )
    )
        .await?
        .json()
        .await
}

pub async fn klines_symbols<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    interval: &str,
    limit: &u32,
    start: &u64,
    end: &u64,
) -> MAP<&'a str, Result<RESULT_KLINE_W, Error_req>>
{
    join_all(
        symbols
        .iter()
        .map(|s| async {
            (s.as_str(), klines(api_url, category, s, interval, limit, start, end).await)
        })
    )
        .await
        .into_iter()
        .collect()
}