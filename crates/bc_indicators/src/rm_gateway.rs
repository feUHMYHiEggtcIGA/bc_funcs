use rustc_hash::FxHashMap;
use bc_utils_lg::enums::indicators::T_HASHMAP;
use bc_utils_lg::structs::src::Src;
use bc_utils_lg::structs::settings::SettingsInd;

#[allow(clippy::wildcard_imports)]
use crate::rm::*;


#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn rm_gw<'a>(
    src: &Src, 
    settings: &'static Vec<SettingsInd>
) -> FxHashMap<&'static str, Vec<T_HASHMAP<'a>>> {
    let mut map = FxHashMap::default();
    
    for setting in settings {
        match setting.key.as_str() {
            "rsi" => {
                let (
                    rm,
                    rm_rma1,
                    rm_rma2,
                ) = rm_rsi(src.open, &setting.kwargs_usize["window"]);
                map.insert(
                    setting.key_uniq.as_str(),
                    vec![
                        T_HASHMAP::Float64(rm),
                        T_HASHMAP::Float64(rm_rma1),
                        T_HASHMAP::Float64(rm_rma2),
                    ]
                );
            }
            _ => panic!("key indication unknown"),
        }
    }
    map
}