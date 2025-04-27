#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_KLINE{
    pub symbol: String,
    pub category: String,
    pub list: Vec<Vec<String>>,
}
