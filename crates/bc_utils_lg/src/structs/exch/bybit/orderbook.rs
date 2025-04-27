#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_ORDERBOOK{
    pub s: String,
    pub a: Vec<Vec<String>>,
    pub b: Vec<Vec<String>>,
    pub ts: i64,
    pub u: i64,
    pub seq: i64,
    pub cts: i64,
}