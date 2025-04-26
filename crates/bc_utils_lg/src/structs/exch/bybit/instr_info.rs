#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_INSTR_INFO_W2_LEVERAGE_FILTER {
    pub minLeverage: String,
    pub maxLeverage: String,
    pub leverageStep: String,
}

#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_INSTR_INFO_W2_PRICE_FILTER {
    pub minPrice: String,
    pub maxPrice: String,
    pub tickSize: String,
}

#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_INSTR_INFO_W2_LOT_SIZE_FILTER {
    pub maxOrderQty: String,
    pub minOrderQty: String,
    pub qtyStep: String,
    pub postOnlyMaxOrderQty: String,
    pub maxMktOrderQty: String,
    pub minNotionalValue: String,
}

#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_INSTR_INFO_W2_RISK_PARAMETERS {
    pub priceLimitRatioX: String,
    pub priceLimitRatioY: String,
}

#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_INSTR_INFO_W1 {
    pub symbol: String,
    pub contractType: String,
    pub status: String,
    pub baseCoin: String,
    pub quoteCoin: String,
    pub launchTime: String,
    pub deliveryTime: String,
    pub deliveryFeeRate: String,
    pub priceScale: String,
    pub leverageFilter: RESULT_INSTR_INFO_W2_LEVERAGE_FILTER,
    pub priceFilter: RESULT_INSTR_INFO_W2_PRICE_FILTER,
    pub lotSizeFilter: RESULT_INSTR_INFO_W2_LOT_SIZE_FILTER,
    pub unifiedMarginTrade: bool,
    pub fundingInterval: i32,
    pub settleCoin: String,
    pub copyTrading: String,
    pub upperFundingRate: String,
    pub lowerFundingRate: String,
    pub isPreListing: bool,
    pub preListingInfo: String,
    pub riskParameters: RESULT_INSTR_INFO_W2_RISK_PARAMETERS,
}

#[derive(Serialize, Deserialize)]
#[derive(std::fmt::Debug)]
pub struct RESULT_INSTR_INFO_W {
    pub category: String,
    pub list: Vec<RESULT_INSTR_INFO_W1>,
    pub nextPageCursor: String,
}