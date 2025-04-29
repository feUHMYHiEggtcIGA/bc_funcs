#![allow(non_camel_case_types)]

use reqwest::{
    Error as Error_req,
    get,
};
use bc_utils_lg::structs::exch::bybit::result::RESULT_EXCH_BYBIT;
use bc_utils_lg::structs::exch::bybit::klines::RESULT_KLINE;
use futures::future::join_all;
use bc_utils_lg::types::maps::MAP;
use bc_utils_core::mechanisms::{
    all_or_nothing, 
    one_time_hm,
};

use crate::bybit::url_const::KLINE;


pub async fn klines_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval: &str,
    limit: &usize,
    start: &usize,
    end: &usize,
) -> Result<RESULT_EXCH_BYBIT<RESULT_KLINE>, Error_req>
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
        .json::<RESULT_EXCH_BYBIT<RESULT_KLINE>>()
    .await
}

pub async fn klines(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval: &str,
    limit: &usize,
    start: &usize,
    end: &usize,
) -> Result<Vec<Vec<String>>, Error_req>
{
    Ok(klines_req(
        api_url, 
        category, 
        symbol, 
        interval, 
        limit, 
        start, 
        end
    ).await?.result.list)
}

pub async fn klines_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval: &str,
    limit: &usize,
    start: &usize,
    end: &usize,
) -> Vec<Vec<String>>
{
    all_or_nothing(
        async || Ok(klines_req(
            api_url,
            category,
            symbol,
            interval,
            limit,
            start,
            end,
        ).await?.result.list),
    ).await
}

pub async fn kline_symbols<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    interval: &str,
) -> MAP<&'a str, Result<Vec<String>, Box<dyn std::error::Error>>>
{
    join_all(
        symbols
           .iter()
           .map(|s| async {
                (
                    s.as_str(), 
                    async {
                        klines(
                            api_url, 
                            category, 
                            s, 
                            interval, 
                            &1, 
                            &0, 
                            &0,
                        )
                            .await?
                            .into_iter()
                            .next()
                            .ok_or(Box::from("not found"))
                    }.await,
                )
           })
    )
        .await
        .into_iter()
        .collect()
}

pub async fn kline_symbols_a<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    interval: &str,
) -> MAP<&'a str, Vec<String>>
{
    join_all(
        symbols
           .iter()
           .map(|s| async {
                (
                    s.as_str(), 
                    async {
                        klines_a(
                            api_url, 
                            category, 
                            s, 
                            interval, 
                            &1, 
                            &0, 
                            &0,
                        )
                            .await
                            .into_iter()
                            .next()
                            .unwrap_or(vec![])
                    }.await,
                )
           })
    )
        .await
        .into_iter()
        .collect()
}

pub async fn kline_symbols_ao<'a>(
    api_url: &'a str,
    category: &'a str,
    symbols: &'a [String],
    interval: &'a str,
) -> MAP<&'a str, Vec<String>>
{   
    one_time_hm(
        async || kline_symbols_a(
            api_url, 
            category, 
            symbols, 
            interval,
        ).await,
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
) -> MAP<&'a str, Result<Vec<Vec<String>>, Error_req>>
{
    join_all(
        symbols
        .iter()
        .map(|s| async {
            (
                s.as_str(), 
                klines(
                    api_url, 
                    category, 
                    s, 
                    interval, 
                    limit, 
                    start, 
                    end
                )
                    .await
            )
        })
    )
        .await
        .into_iter()
        .collect()
}

pub async fn klines_symbols_a<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    interval: &str,
    limit: &usize,
    start: &usize,
    end: &usize,
) -> MAP<&'a str, Vec<Vec<String>>>
{
    join_all(
        symbols
        .iter()
        .map(|s| async {
            (
                s.as_str(), 
                klines_a(
                    api_url, 
                    category, 
                    s, 
                    interval, 
                    limit, 
                    start, 
                    end
                )
                    .await
            )
        })
    )
        .await
        .into_iter()
        .collect()
}
