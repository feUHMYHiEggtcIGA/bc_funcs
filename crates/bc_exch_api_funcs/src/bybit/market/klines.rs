#![allow(non_camel_case_types)]

use reqwest::{
    Error as Error_req,
    get,
};
use bc_utils_lg::{enums::indicators::T_ARGS, structs::exch::bybit::klines::RESULT_KLINE_W};
use futures::future::join_all;
use bc_utils_lg::types::maps_abstr::MAP;
use bc_core_funcs::mechanisms::all_or_nothing;
// use bc_utils_lg::enums::indicators::T_ARGS

use crate::bybit::api_url::KLINE;


pub async fn klines(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval: &str,
    limit: &usize,
    start: &usize,
    end: &usize,
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

pub async fn klines_aon(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval: &str,
    limit: &usize,
    start: &usize,
    end: &usize,
) -> RESULT_KLINE_W
{
    all_or_nothing(
        |v| klines(
            v[0].unwrap_str(),
            v[1].unwrap_str(),
            v[2].unwrap_str(),
            v[3].unwrap_str(),
            v[4].unwrap_usize(),
            v[5].unwrap_usize(),
            v[6].unwrap_usize(),
        ),
        &vec![
            T_ARGS::Str(api_url),
            T_ARGS::Str(category),
            T_ARGS::Str(symbol),
            T_ARGS::Str(interval),
            T_ARGS::Usize(*limit),
            T_ARGS::Usize(*start),
            T_ARGS::Usize(*end),
        ],
    ).await
}

pub async fn klines_symbols<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    interval: &str,
    limit: &usize,
    start: &usize,
    end: &usize,
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