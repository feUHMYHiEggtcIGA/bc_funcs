#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct RESULT_ACC_INFO{
    pub marginMode: String,
    pub updatedTime: String,
    pub unifiedMarginStatus: i32,
    pub dcpStatus: String,
    pub timeWindow: i32,
    pub smpGroup: i32,
    pub isMasterTrader: bool,
    pub spotHedgingStatus: String,
}