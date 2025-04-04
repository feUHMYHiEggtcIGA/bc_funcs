#![allow(non_camel_case_types)]

use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct SETTINGS_USED_MODS {
    pub key: String,
    pub kwargs_usize: FxHashMap<String, usize>,
    pub kwargs_f64: FxHashMap<String, f64>,
    pub kwargs_string: FxHashMap<String, String>,
    pub used_indications: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SETTINGS_USED_SRC {
    pub key: String,
    pub key_uniq: String,
    pub is_past: bool, 
    pub used_mods: Vec<SETTINGS_USED_MODS>
}

#[derive(Serialize, Deserialize)]
pub struct SETTINGS_IND {
    pub key: String,
    pub key_uniq: String,
    pub kwargs_usize: FxHashMap<String, usize>,
    pub kwargs_f64: FxHashMap<String, f64>,
    pub kwargs_string: FxHashMap<String, String>,
    pub used_src: Vec<SETTINGS_USED_SRC>,
    pub used_mods: Vec<SETTINGS_USED_MODS>,
}

#[derive(Serialize, Deserialize)]
pub struct SETTINGS {
    pub exch_api_key: String,
    pub exch_api_secret: String,
    pub msg_key: String,
    pub msg_chat: String,
    pub is_demo: bool,
    pub indications: Vec<SETTINGS_IND>,
}
