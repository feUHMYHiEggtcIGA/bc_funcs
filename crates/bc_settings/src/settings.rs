use std::sync::LazyLock;
use std::fs::File;
use std::io::BufReader;

use bc_utils_lg::structs::settings::Settings;
use serde_json;

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| {
    let reader = BufReader::new(File::open("../../settings.json").expect("file not found"));
    serde_json::from_reader(reader).expect("settings.json is not decerialized")
});