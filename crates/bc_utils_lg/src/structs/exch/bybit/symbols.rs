#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_SYMBOLS1 {
    pub symbol: String,
    pub lastPrice: String,
    pub indexPrice: String,
    pub markPrice: String,
    pub prevPrice24h: String,
    pub price24hPcnt: String,
    pub highPrice24h: String,
    pub lowPrice24h: String,
    pub prevPrice1h: String,
    pub openInterest: String,
    pub openInterestValue: String,
    pub turnover24h: String,
    pub volume24h: String,
    pub fundingRate: String,
    pub nextFundingTime: String,
    pub predictedDeliveryPrice: String,
    pub basisRate: String,
    pub deliveryFeeRate: String,
    pub deliveryTime: String,
    pub ask1Size: String,
    pub bid1Price: String,
    pub ask1Price: String,
    pub bid1Size: String,
    pub basis: String,
}

#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct  RESULT_SYMBOLS {
    pub category: String,
    pub list: Vec<RESULT_SYMBOLS1>,
}