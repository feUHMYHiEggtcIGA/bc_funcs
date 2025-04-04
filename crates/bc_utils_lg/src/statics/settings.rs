use std::sync::LazyLock;

use rustc_hash::FxHashMap;

use crate::structs::settings::{
    SETTINGS_IND, 
    SETTINGS_USED_MODS, SETTINGS_USED_SRC,
};


pub static SETTINGS_IND_TEST: LazyLock<Vec<SETTINGS_IND>> = LazyLock::new(|| {
    vec![
        SETTINGS_IND{
            key: String::from("rsi"),
            key_uniq: String::from("rsi_1"),
            kwargs_usize: FxHashMap::from_iter([("window".to_string(), 2)]),
            kwargs_f64: FxHashMap::default(),
            kwargs_string: FxHashMap::default(),
            used_src: vec![
                SETTINGS_USED_SRC{
                    key: "open".to_string(), 
                    key_uniq: "open_1".to_string(), 
                    is_past: false, 
                    used_mods: vec![],
                }
            ],
            used_mods: vec![],
        },
        SETTINGS_IND{
            key: String::from("rsi"),
            key_uniq: String::from("rsi_2"),
            kwargs_usize: FxHashMap::from_iter([("window".to_string(), 3)]),
            kwargs_f64: FxHashMap::default(),
            kwargs_string: FxHashMap::default(),
            used_src: vec![],
            used_mods: vec![
                SETTINGS_USED_MODS{
                    key: String::from("avg"),
                    kwargs_usize: FxHashMap::default(),
                    kwargs_f64: FxHashMap::default(),
                    kwargs_string: FxHashMap::default(),
                    used_indications: vec![String::from("rsi_1")],
                }
            ],
        },
        SETTINGS_IND{
            key: String::from("sma"),
            key_uniq: String::from("sma_1"),
            kwargs_usize: FxHashMap::from_iter([("window".to_string(), 2)]),
            kwargs_f64: FxHashMap::default(),
            kwargs_string: FxHashMap::default(),
            used_src: vec![],
            used_mods: vec![
                SETTINGS_USED_MODS{
                    key: String::from("nohesi"),
                    kwargs_usize: FxHashMap::default(),
                    kwargs_f64: FxHashMap::from_iter([("hesi".to_string(), 0.0001)]),
                    kwargs_string: FxHashMap::default(),
                    used_indications: vec![],
                }
            ],
        },
    ]
});

pub static SETTINGS_RSI_EMPTY: LazyLock<Vec<SETTINGS_IND>> = LazyLock::new(|| {
    vec![
        SETTINGS_IND{
            key: String::from("rsi"),
            key_uniq: String::from("rsi_1"),
            kwargs_usize: FxHashMap::from_iter([("window".to_string(), 2)]),
            kwargs_f64: FxHashMap::default(),
            kwargs_string: FxHashMap::default(),
            used_src: vec![
                SETTINGS_USED_SRC{
                    key: "open".to_string(), 
                    key_uniq: "open_1".to_string(), 
                    is_past: false, 
                    used_mods: vec![],
                }
            ],
            used_mods: vec![],
        },
    ]
});

pub const WINDOW: usize = 2;