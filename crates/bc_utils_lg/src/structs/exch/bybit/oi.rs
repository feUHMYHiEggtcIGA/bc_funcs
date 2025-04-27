#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_OI1{
    pub openInterest: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_OI{
    pub symbol: String,
    pub category: String,
    pub list: Vec<RESULT_OI1>,
}