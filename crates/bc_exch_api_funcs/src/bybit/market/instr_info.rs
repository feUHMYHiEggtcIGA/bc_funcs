use reqwest::{get, Error as Error_req};
use bc_utils_lg::structs::exch::bybit::instr_info::{
    RESULT_INSTR_INFO,
    RESULT_INSTR_INFO1, 
};
use bc_utils_lg::structs::exch::bybit::result::RESULT_EXCH_BYBIT;
use bc_core_funcs::mechanisms::all_or_nothing;
use bc_utils_lg::types::maps::MAP;

use crate::bybit::url_const::INSTR_INFO;


pub async fn instr_info_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    status: &str,
    base_coin: &str,
    limit: &usize,
    cursor: &str
) -> Result<RESULT_EXCH_BYBIT<RESULT_INSTR_INFO>, Error_req>
{
    get(format!(
        "{api_url}{INSTR_INFO}\
        ?category={category}\
        &symbol={symbol}\
        &status={status}\
        &baseCoin={base_coin}\
        &limit={limit}\
        &cursor={cursor}"
    ))
        .await?
        .json::<RESULT_EXCH_BYBIT<RESULT_INSTR_INFO>>()
        .await
}

pub async fn instr_info(
    api_url: &str,
    category: &str,
    symbol: &str,
    status: &str,
    base_coin: &str,
) -> Result<RESULT_INSTR_INFO1, Box<dyn std::error::Error>>
{
    instr_info_req(
        api_url, 
        category, 
        symbol, 
        status, 
        base_coin, 
        &1,
        ""
    ).await?.result.list.into_iter().next().ok_or(Box::from("not found"))
}

pub async fn instr_info_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    status: &str,
    base_coin: &str,
) -> RESULT_INSTR_INFO1
{
    all_or_nothing(
        async || instr_info(
            api_url, 
            category, 
            symbol, 
            status, 
            base_coin, 
        ).await
    ).await
}

pub async fn instrs_info<'a>(
    api_url: &'a str,
    category: &'a str,
    symbols: &'a [String],
    status: &'a str,
    base_coin: &'a str,
) -> Result<MAP<&'a str, RESULT_INSTR_INFO1>, Box<dyn std::error::Error>> 
{
    let mut res = MAP::default();
    let mut passed = vec![];
    let mut cursor = "".to_string();
    while passed.len() != symbols.len() {
        let response_ = instr_info_req(
            api_url, 
            category, 
            "", 
            status, 
            base_coin, 
            // fix this `limit` arg ↓
            &1000,
            &cursor
        )
            .await?.result;
        cursor = response_.nextPageCursor.clone();
        for v in  response_.list.into_iter(){
            for s in symbols {
                if s == &v.symbol {
                    res.insert(s.as_str(), v);
                    passed.push(s.as_str());
                    break;
                }
            }
        }
    }
    Ok(res)
}

pub async fn instrs_info_a<'a>(
    api_url: &'a str,
    category: &'a str,
    symbols: &'a [String],
    status: &'a str,
    base_coin: &'a str,
) -> MAP<&'a str, RESULT_INSTR_INFO1>
{
    all_or_nothing(
        || instrs_info(
            api_url, 
            category, 
            symbols, 
            status, 
            base_coin
        )
    ).await
}