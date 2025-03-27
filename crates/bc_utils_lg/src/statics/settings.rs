use std::sync::LazyLock;

use rustc_hash::FxHashMap;

use crate::structs::settings::SettingsInd;


pub static SETTINGS_IND_TEST: LazyLock<Vec<SettingsInd>> = LazyLock::new(|| vec![SettingsInd{
    key: String::from("rsi"),
    key_uniq: String::from("rsi_1"),
    kwargs_usize: FxHashMap::from_iter([(String::from("window"), 2)]),
    kwargs_f64: FxHashMap::default(),
    kwargs_string: FxHashMap::default(),
    used_src: vec![],
    used_indications: vec![],
    used_mods: vec![],
    execution_on_count: 0,
}]);

pub const WINDOW: usize = 2;