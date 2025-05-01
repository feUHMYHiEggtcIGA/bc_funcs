#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct RESULT_WALLET_BALANCE2 {
    pub availableToBorrow: String,
    pub bonus: String,
    pub accruedInterest: String,
    pub availableToWithdraw: String,
    pub totalOrderIM: String,
    pub equity: String,
    pub totalPositionMM: String,
    pub usdValue: String,
    pub spotHedgingQty: String,
    pub unrealisedPnl: String,
    pub collateralSwitch: bool,
    pub borrowAmount: String,
    pub totalPositionIM: String,
    pub walletBalance: String,
    pub cumRealisedPnl: String,
    pub locked: String,
    pub marginCollateral: bool,
    pub coin: String
}

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct RESULT_WALLET_BALANCE1 {
    pub totalEquity: String,
    pub accountIMRate: String,
    pub totalMarginBalance: String,
    pub totalInitialMargin: String,
    pub accountType: String,
    pub totalAvailableBalance: String,
    pub accountMMRate: String,
    pub totalPerpUPL: String,
    pub totalWalletBalance: String,
    pub accountLTV: String,
    pub totalMaintenanceMargin: String,
    pub coin: Vec<RESULT_WALLET_BALANCE2>,
}

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct RESULT_WALLET_BALANCE {
    pub list: Vec<RESULT_WALLET_BALANCE1>,
}