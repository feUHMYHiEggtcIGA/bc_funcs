use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct SETTINGS_USED_SRC {
    pub key: String,
    pub slice_or_i: Vec<i64>
}

#[derive(Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct SETTINGS_USED_IND_MODS {
    pub key: String,
    pub kwargs_usize: FxHashMap<String, usize>,
    pub kwargs_f64: FxHashMap<String, f64>,
    pub kwargs_string: FxHashMap<String, String>,
    pub used_indications: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct SETTINGS_IND {
    pub key: String,
    pub key_uniq: String,
    pub kwargs_usize: FxHashMap<String, usize>,
    pub kwargs_f64: FxHashMap<String, f64>,
    pub kwargs_string: FxHashMap<String, String>,
    pub used_src: Vec<SETTINGS_USED_SRC>,
    pub used_indications: Vec<SETTINGS_USED_IND_MODS>,
    pub used_mods: Vec<SETTINGS_USED_IND_MODS>,
    pub execution_on_count: usize,
}

#[derive(Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Settings {
    pub exch_api_key: String,
    pub exch_api_secret: String,
    pub msg_key: String,
    pub msg_chat: String,
    pub is_demo: bool,
    pub indications: Vec<SETTINGS_IND>,
}
