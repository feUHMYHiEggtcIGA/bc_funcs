use std::sync::LazyLock;

use rustc_hash::FxHashMap;

use crate::structs::settings::{
    SETTINGS_IND, 
    SETTINGS_USED_IND_MODS,
};


pub static SETTINGS_IND_TEST: LazyLock<Vec<SETTINGS_IND>> = LazyLock::new(|| {
    vec![
        SETTINGS_IND{
            key: String::from("rsi"),
            key_uniq: String::from("rsi_1"),
            kwargs_usize: FxHashMap::from_iter([(String::from("window"), 2)]),
            kwargs_f64: FxHashMap::default(),
            kwargs_string: FxHashMap::default(),
            used_src: vec![],
            used_indications: vec![],
            used_mods: vec![],
            execution_on_count: 0,
        },
        // SETTINGS_IND{
        //     key: String::from("rsi"),
        //     key_uniq: String::from("rsi_2"),
        //     kwargs_usize: FxHashMap::from_iter([(String::from("window"), 3)]),
        //     kwargs_f64: FxHashMap::default(),
        //     kwargs_string: FxHashMap::default(),
        //     used_src: vec![],
        //     used_indications: vec![],
        //     used_mods: vec![
        //         SETTINGS_USED_IND_MODS{
        //             key: String::from("avg"),
        //             kwargs_usize: FxHashMap::default(),
        //             kwargs_f64: FxHashMap::default(),
        //             kwargs_string: FxHashMap::default(),
        //             used_indications: vec![String::from("rsi_1")]
        //         }
        //     ],
        //     execution_on_count: 1,
        // }
    ]
});

pub const WINDOW: usize = 2;