#![allow(non_camel_case_types)]

use reqwest::{
    Error as Error_req,
    get,
};
use bc_utils_lg::structs_and_types::{
    exch::bybit::klines::RESULT_KLINE_W, 
    structures_abstr::ARGS
};
use bc_utils_lg::enums::indicators::T_ARGS;
use futures::future::join_all;
use bc_utils_lg::structs_and_types::maps_abstr::MAP;
use bc_core_funcs::mechanisms::{
    all_or_nothing, 
    one_time_hm,
    one_time_hm_kline,
};

use crate::bybit::api_url::KLINE;


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
    Ok(
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
            .json::<RESULT_KLINE_W>()
        .await?
        .result
        .list
    )
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

pub async fn kline_symbols<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    interval: &str,
) -> MAP<&'a str, Result<Vec<String>, Error_req>>
{
    join_all(
        symbols
           .iter()
           .map(|s| async {
                (s.as_str(), Ok(klines(
                    api_url, 
                    category, 
                    s, 
                    interval, 
                    &1, 
                    &0, 
                    &0,
                ).await.unwrap()[0].clone()))
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
            (s.as_str(), klines_a(
                api_url, 
                category, 
                s, 
                interval,
                &1,
                &0, 
                &0,
            ).await[0].clone())
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
    one_time_hm_kline(
        kline_symbols_a,
        (api_url, category, symbols, interval),
    ).await
}

pub async fn kline_symbols_ao_abstr<'a>(
    args: &'a ARGS<'a, f64, String>,
) -> MAP<&'a str, Vec<String>>
{   
    one_time_hm(
        |v| kline_symbols_a(
            v[0].unwrap_str(),
            v[1].unwrap_str(),
            v[2].unwrap_slice(),
            v[3].unwrap_str(),
        ),
        args,
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
            (s.as_str(), klines(api_url, category, s, interval, limit, start, end).await)
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
            (s.as_str(), klines_a(api_url, category, s, interval, limit, start, end).await)
        })
    )
        .await
        .into_iter()
        .collect()
}
