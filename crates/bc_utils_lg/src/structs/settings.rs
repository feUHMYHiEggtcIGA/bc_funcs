#![allow(non_camel_case_types)]

use crate::types::maps::MAP;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct SETTINGS_USED_SRC {
    pub key: String,
    pub sub_from_last_i: usize, 
}

#[derive(Serialize, Deserialize)]
pub struct SETTINGS_IND {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_SRC>,
    pub used_ind: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SETTINGS_EXCH {
    pub api_url: String,
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct SETTINGS_MSG {
    pub key: String,
    pub chat: String,
}

#[derive(Serialize, Deserialize)]
pub struct SETTINGS {
    pub exch: SETTINGS_EXCH,
    pub msg: SETTINGS_MSG,
    pub indications: MAP<String, SETTINGS_IND>,
}
