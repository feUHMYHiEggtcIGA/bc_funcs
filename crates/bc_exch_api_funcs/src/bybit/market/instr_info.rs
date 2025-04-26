use reqwest::{get, Error as Error_req};
use bc_utils_lg::structs::exch::bybit::instr_info::RESULT_INSTR_INFO_W;
use bc_utils_lg::structs::exch::bybit::result::RESULT_EXCH_BYBIT;

use crate::bybit::url_const::INSTR_INFO;


pub async fn instr_info(
    api_url: &str,
    category: &str,
    symbol: &str,
    status: &str,
    base_coin: &str,
    limit: &usize,
    cursor: &str
) -> Result<RESULT_INSTR_INFO_W, Error_req>
{
    Ok(get(format!(
        "{api_url}{INSTR_INFO}\
        ?category={category}\
        ?symbol={symbol}\
        ?status={status}\
        ?baseCoin={base_coin}\
        ?limit={limit}\
        ?cursor={cursor}"
    ))
        .await?
        .json::<RESULT_EXCH_BYBIT<RESULT_INSTR_INFO_W>>()
        .await?
        .result)
}