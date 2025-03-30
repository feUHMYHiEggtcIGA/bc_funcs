use std::ops::{AddAssign, DivAssign};

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::enums::indicators::T_HASHMAP;
use bc_utils_lg::structs::src::SRC;
use bc_utils_lg::structs::settings::SETTINGS_IND;

#[allow(clippy::wildcard_imports)]
use crate::rm::*;


#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn rm_gw<'a, T>(
    src: &SRC<T>,
    settings: &'static Vec<SETTINGS_IND>
) -> FxHashMap<&'static str, Vec<T_HASHMAP<'a, T>>> 
where
    T: Float,
    T: AddAssign,
    T: DivAssign,
{
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
                        T_HASHMAP::Float(rm),
                        T_HASHMAP::Float(rm_rma1),
                        T_HASHMAP::Float(rm_rma2),
                    ]
                );
            }
            _ => panic!("key indication unknown"),
        }
    }
    map
}