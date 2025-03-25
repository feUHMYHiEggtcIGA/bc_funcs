use std::sync::OnceLock;

use rustc_hash::FxHashMap;
use bc_utils_lg::structs::src::SrcEl;
use bc_utils_lg::structs::settings::SettingsInd;
#[allow(clippy::wildcard_imports)]
use bc_utils_lg::enums::indicators::*;

// use crate::indicators::no_oscillators::trend::*;
// use crate::indicators::no_oscillators::other::*;
// use crate::indicators::oscillators::multipliers::*;
#[allow(clippy::wildcard_imports)]
use crate::indicators::oscillators::trend::*;
#[allow(clippy::wildcard_imports)]
use crate::indicators::oscillators::other::*;
// use crate::common::*;


static MAP_ARGS_RM: OnceLock<FxHashMap<&'static str, Vec<T_ARGS>>> = OnceLock::new();
#[allow(clippy::type_complexity)]
static MAP_INDICATORS_RM: OnceLock<
    FxHashMap<&'static str, fn(&SrcEl, &Vec<T_ARGS>,  &mut Vec<T_HASHMAP>) -> f64>
> = OnceLock::new();

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
pub fn map_args_rm_init(settings: &'static Vec<SettingsInd>) {
    MAP_ARGS_RM.set(
            settings.iter().map(|setting| (
                setting.key.as_str(),
                match setting.key.as_str() {
                    "rsi" => vec![T_ARGS::None(())],
                    "tqo_b" => vec![
                        T_ARGS::Usize(setting.kwargs_usize["window_noise"]),
                        T_ARGS::Float64(setting.kwargs_f64["correlation_factor"]),
                        T_ARGS::String(setting.kwargs_string["noise_type"].clone()),
                    ],
                    _ => panic!("key indication unknown"),
                }
            )
        ).collect()
    )
        .expect("args map not initialized. maybe it was initialized earlier");
}


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
pub fn map_indicators_rm_init(settings: &'static Vec<SettingsInd>) {
    MAP_INDICATORS_RM.set(settings.iter().map(|v| (
            v.key.as_str(),
            match v.key.as_str() {
                "rsi" => |src: &SrcEl, _: &Vec<T_ARGS>, rm:  &mut Vec<T_HASHMAP>| {
                    let mut v1: Option<&mut FxHashMap<&'static str, f64>> = None;
                    let mut v2: Option<&mut FxHashMap<&'static str, f64>> = None;
                    let mut v3: Option<&mut FxHashMap<&'static str, f64>> = None;
                    for (i, map) in rm
                        .iter_mut()
                        .enumerate() {
                        match (i, map) {
                            (0, T_HASHMAP::Float64(map)) => v1 = Some(map),
                            (1, T_HASHMAP::Float64(map)) => v2 = Some(map),
                            (2, T_HASHMAP::Float64(map)) => v3 = Some(map),
                            _ => panic!("map not initialized. maybe it was initialized earlier"),
                        }
                    }
                    if let (
                        Some(v1), 
                        Some(v2), 
                        Some(v3)
                    ) = (v1, v2, v3) {
                        rsi_rm(
                            &src.open,  
                            v1, 
                            v2,
                            v3
                        )
                    } else {
                        panic!("map not initialized. maybe it was initialized earlier")
                    }
                },
                "tqo_b" => |src: &SrcEl, v: &Vec<T_ARGS>, rm:  &mut Vec<T_HASHMAP>| {
                    let (
                        v1, 
                        v2,
                        v3,
                    ) = (v[0].unwrap_usize(), v[1].unwrap_f64(), v[2].unwrap_string());
                    let mut v4: Option<&mut FxHashMap<&'static str, f64>> = None;
                    let mut v5: Option<&mut FxHashMap<&'static str, f64>> = None;
                    let mut v6: Option<&mut FxHashMap<&'static str, f64>> = None;
                    let mut v7: Option<&mut FxHashMap<&'static str, Vec<f64>>> = None;
                    for (i, map) in rm
                        .iter_mut()
                        .enumerate() {
                        match (i, map) {
                            (0, T_HASHMAP::Float64(map)) => v4 = Some(map),
                            (1, T_HASHMAP::Float64(map)) => v5 = Some(map),
                            (2, T_HASHMAP::Float64(map)) => v6 = Some(map),
                            (3, T_HASHMAP::VecF64(map)) => v7 = Some(map),
                            _ => panic!("map not initialized. maybe it was initialized earlier"),
                        }
                    }
                    if let (Some(v4), Some(v5), Some(v6), Some(v7)) = (v4, v5, v6, v7) {
                        tqo_b_rm(
                            &src.open,
                            v1,
                            v2,
                            v3,
                            v4,
                            v5,
                            v6,
                            v7,
                        )
                    } else {
                        panic!("map not initialized. maybe it was initialized earlier")
                    }
                },
                _ => panic!("key indication unknown"),
            })
        ).collect()
    )
        .expect("args map not initialized. maybe it was initialized earlier");
}


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::implicit_hasher)]
#[allow(clippy::ptr_arg)]
pub fn indications_gw_rm(
    src: &SrcEl,
    settings: &'static Vec<SettingsInd>,
    rm: &mut FxHashMap<&'static str, Vec<T_HASHMAP>> 
) -> FxHashMap<&'static str, f64>
{ 
    settings
        .iter()
        .map(
            |v| 
            (
                v.key_uniq.as_str(),
                // 0.0
                MAP_INDICATORS_RM
                    .get()
                    .expect("indicators not found in map indicators")[&v.key.as_str()](
                        src, 
                        &MAP_ARGS_RM.get().expect("map args rm not initializeted")[&v.key.as_str()], 
                        rm.get_mut(v.key.as_str()).expect("rm not found"),
                )
            )
        )
        .collect()
}