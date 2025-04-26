#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_EXCH_BYBIT<T> {
    pub retCode: i32,
    pub retMsg: String,
    pub result: T,
}