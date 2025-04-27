use reqwest::{get, Error as Error_req};
use bc_utils_lg::structs::exch::bybit::oi::{
    RESULT_OI,
    RESULT_OI1, 
};
use bc_utils_lg::structs::exch::bybit::result::RESULT_EXCH_BYBIT;
use bc_core_funcs::mechanisms::all_or_nothing;

use crate::bybit::url_const::OI;


pub async fn oi_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval_time: &str,
    start_time: &usize,
    end_time: &usize,
    limit: &usize,
    cursor: &str,
) -> Result<RESULT_EXCH_BYBIT<RESULT_OI>, Error_req>
{
    
    get(format!("\
        {api_url}\
        {OI}\
        ?category={category}\
        &symbol={symbol}\
        &intervalTime={interval_time}\
        &startTime={start_time}\
        &endTime={end_time}\
        &limit={limit}\
        &cursor={cursor}\
    ")
    )
        .await?
        .json()
        .await
}

pub async fn oi(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval_time: &str,
    start_time: &usize,
    end_time: &usize,
    limit: &usize,
    cursor: &str,
) -> Result<Vec<RESULT_OI1>, Box<dyn std::error::Error>>
{
    Ok(oi_req(
        api_url, 
        category, 
        symbol, 
        interval_time, 
        start_time, 
        end_time, 
        limit, 
        cursor
    ).await?.result.list)
}

pub async fn oi_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    interval_time: &str,
    start_time: &usize,
    end_time: &usize,
    limit: &usize,
    cursor: &str,
) -> Vec<RESULT_OI1>
{
    all_or_nothing(||oi(
        api_url, 
        category, 
        symbol, 
        interval_time, 
        start_time, 
        end_time, 
        limit, 
        cursor
    )).await
}

