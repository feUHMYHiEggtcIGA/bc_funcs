#![allow(non_camel_case_types)]

use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;

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

use crate::bybit::const_url::KLINE;


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

/// the function returns values from the beginning of the start to the end (in ascending order)
/// It's a cumbersome implementation, but I don't want to fuck with it right now.
pub async fn klines(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval: &str,
    limit: &usize,
    start: &usize,
    end: &usize,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>>
{
    let inter = match interval {
        "D" => 1440,
        "W" => 10_080,
        "M" => 43_200,
        v => v.parse::<usize>().unwrap(),
    };
    let time_stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as usize;
    let changes = inter * limit * 60 * 1000;
    let step = inter * 1000 * 60 * 1000;
    let mut limits_num = *limit;
    let mut limits = vec![];
    let mut time1 = vec![];
    let mut time2 = vec![];
    let diff_st_end = end - start;
    for time_  in ({
        if diff_st_end != changes && start == &0 && end == &0 {
            time_stamp - changes
        } else if diff_st_end != changes && start == &0 && end != &0{
            *end - changes
        } else {
            *start
        }
    }..{
        if diff_st_end != changes && start == &0 && end == &0 {
            time_stamp
        } else if diff_st_end != changes && start != &0 && end == &0{
            start + changes
        } else {
            *end
        }
    })
        .rev()
        .step_by(step)
    {
        let sk = match limits_num % 1000 {
            0 => 1000,
            v => v
        };
        limits_num -= sk;
        limits.push(sk);
        time1.push(time_ - step * 2);
        time2.push(time_ - step);
        if limits_num == 0 {
            break;
        }
    }
    Ok(join_all(
        time1
            .iter()
            .zip(time2.iter())
            .zip(limits.iter())
            .map(|((time1_, time2_), l,)| klines_req(
                api_url, 
                category, 
                symbol, 
                interval, 
                l,
                time1_,
                time2_,
        ))
    )
        .await
        .into_iter()
        .map(|v| -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
            let mut sk = v?.result.list;
            sk.reverse();
            Ok(sk)
        })
        .collect::<Result<Vec<Vec<Vec<String>>>, Box<dyn std::error::Error>>>()?
        .concat())
}

pub async fn klines_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval: &str,
    limit: &usize,
    start: &usize,
    end: &usize,
    wait_sec: &f64,
) -> Result<Vec<Vec<String>>, Box<dyn Error>>
{
    all_or_nothing(
        async || klines(
            api_url,
            category,
            symbol,
            interval,
            limit,
            start,
            end,
        ).await,
        wait_sec,
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
    wait_sec: &f64
) -> Result<MAP<&'a str, Vec<String>>, Box<dyn Error>>
{
    join_all(
        symbols
           .iter()
           .map(|s| async {
                Ok((
                    s.as_str(), 
                    async {
                        match klines_a(
                            api_url, 
                            category, 
                            s, 
                            interval, 
                            &1, 
                            &0, 
                            &0,
                            wait_sec,
                        )
                            .await
                            {
                                Ok(v) => Ok(v.into_iter()
                            .next()
                            .unwrap_or(vec![])),
                                Err(_) => Err(Box::<dyn Error>::from("klines_a err"))
                            }
                            
                           
                    }.await?,
                ))
           })
    )
        .await
        .into_iter()
        .collect::<Result<_, Box<dyn Error>>>()
}

pub async fn kline_symbols_ao<'a>(
    api_url: &'a str,
    category: &'a str,
    symbols: &'a [String],
    interval: &'a str,
    wait_sec: &f64,
) -> Result<MAP<&'a str, Vec<String>>, Box<dyn Error>>
{   
    one_time_hm(
        async || kline_symbols_a(
            api_url, 
            category, 
            symbols, 
            interval,
            wait_sec,
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
) -> MAP<&'a str, Result<Vec<Vec<String>>, Box<dyn std::error::Error>>>
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
    wait_sec: &f64,
) -> Result<MAP<&'a str, Vec<Vec<String>>>, Box<dyn Error>>
{
    join_all(
        symbols
        .iter()
        .map(|s| async {
            Ok((
                s.as_str(), 
                klines_a(
                    api_url, 
                    category, 
                    s, 
                    interval, 
                    limit, 
                    start, 
                    end,
                    wait_sec,
                )
                    .await?
            ))
        })
    )
        .await
        .into_iter()
        .collect::<Result<_, Box<dyn Error>>>()
}
