use std::sync::LazyLock;
use std::fs::File;
use std::io::BufReader;
use std::vec;

use crate::types::maps::{
    MAP_LINK,
    MAP,
};
use crate::structs::settings::{
    SETTINGS_IND, 
    SETTINGS_USED_SRC,
    SETTINGS as SETTINGS_STRUCT,
};


pub static SETTINGS_IND_TEST: LazyLock<MAP_LINK<String, SETTINGS_IND>> = LazyLock::new(|| {
    [
        (
            "src_1".to_string(),
            SETTINGS_IND{
                key: String::from("avg"),
                kwargs_usize: MAP::default(),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![
                    SETTINGS_USED_SRC{
                        key: "open".to_string(),
                        sub_from_last_i: 0,
                    },
                    SETTINGS_USED_SRC{
                        key: "high".to_string(),
                        sub_from_last_i: 1,
                    },
                    SETTINGS_USED_SRC{
                        key: "low".to_string(),
                        sub_from_last_i: 1,
                    }
                ],
                used_ind: vec![],
            }
        ),
        (
            "rsi_1".to_string(),
            SETTINGS_IND{
                key: String::from("rsi"),
                kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![],
                used_ind: vec!["src_1".to_string()],
            },
        ),
        (
            "rsi_2".to_string(),
            SETTINGS_IND{
                key: String::from("rsi"),
                kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![],
                used_ind: vec!["rsi_1".to_string()],
            },
        ),
        (
            "ind".to_string(),
            SETTINGS_IND{
                key: String::from("avg"),
                kwargs_usize: MAP::default(),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![
                    SETTINGS_USED_SRC{
                        key: "open".to_string(),
                        sub_from_last_i: 1,
                    }
                ],
                used_ind: vec!["rsi_2".to_string()],
            },
        ),
    ]
        .into_iter()
        .collect()
});

pub static SETTINGS_RSI_EMPTY: LazyLock<MAP_LINK<String, SETTINGS_IND>> = LazyLock::new(|| {
    [
        (
            "rsi_1".to_string(),
            SETTINGS_IND{
                key: String::from("rsi"),
                kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![
                    SETTINGS_USED_SRC{
                        key: "open".to_string(),
                        sub_from_last_i: 0,
                    }
                ],
                used_ind: vec![],
            },
        )
    ]
        .into_iter()
        .collect()
});

pub static SETTINGS: LazyLock<SETTINGS_STRUCT> = LazyLock::new(|| {
    // change_this: path
    let reader = BufReader::new(File::open("../../settings.json").expect("file not found"));
    serde_json::from_reader(reader).expect("settings.json is not decerialized")
});

pub const WINDOW: usize = 2;