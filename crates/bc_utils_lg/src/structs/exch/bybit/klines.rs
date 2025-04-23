#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
struct RESULT_KLINE{
    symbol: String,
    category: String,
    list: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_KLINE_W{
    retCode: i32,
    retMsg: String,
    result: RESULT_KLINE,
}
