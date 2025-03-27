use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct SettingsUsedSrc {
    pub key: String,
    pub slice_or_i: Vec<i64>
}

#[derive(Serialize, Deserialize)]
pub struct SettingsUsedIndMods {
    pub key: String,
    pub kwargs_usize: FxHashMap<String, usize>,
    pub kwargs_f64: FxHashMap<String, f64>,
    pub kwargs_string: FxHashMap<String, String>,
    pub used_indications: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SettingsInd {
    pub key: String,
    pub key_uniq: String,
    pub kwargs_usize: FxHashMap<String, usize>,
    pub kwargs_f64: FxHashMap<String, f64>,
    pub kwargs_string: FxHashMap<String, String>,
    pub used_src: Vec<SettingsUsedSrc>,
    pub used_indications: Vec<SettingsUsedIndMods>,
    pub used_mods: Vec<SettingsUsedIndMods>,
    pub execution_on_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub exch_api_key: String,
    pub exch_api_secret: String,
    pub msg_key: String,
    pub msg_chat: String,
    pub is_demo: bool,
    pub indications: Vec<SettingsInd>,
}
