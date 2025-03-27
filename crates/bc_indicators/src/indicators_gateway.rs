use core::panic;

use rustc_hash::FxHashMap;
use bc_utils_lg::structs::src::SrcEl;
use bc_utils_lg::structs::settings::SettingsInd;
#[allow(clippy::wildcard_imports)]
use bc_utils_lg::enums::indicators::*;
use bc_utils_lg::types::indicators::*;

#[allow(clippy::wildcard_imports)]
use crate::indicators::no_oscillators::trend::*;
#[allow(clippy::wildcard_imports)]
use crate::indicators::oscillators::trend::*;
#[allow(clippy::wildcard_imports)]
use crate::indicators::oscillators::other::*;


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
#[must_use]
pub fn map_args_rm(settings: &'static Vec<SettingsInd>) -> MAP_ARGS_RM
{
    settings
        .iter()
        .map(
            |setting| 
            (
                setting.key_uniq.as_str(),
                match setting.key.as_str() {
                    "sma" => vec![T_ARGS::Usize(setting.kwargs_usize["window"])],
                    "rma" | "ema" | "rsi" => vec![T_ARGS::None(())],
                    "tqo_b" => vec![
                        T_ARGS::Usize(setting.kwargs_usize["window_noise"]),
                        T_ARGS::Float64(setting.kwargs_f64["correlation_factor"]),
                        T_ARGS::String(setting.kwargs_string["noise_type"].clone()),
                    ],
                    _ => panic!("key indication unknown"),
                }
            )
        )
        .collect()
}


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
#[must_use]
pub fn map_indicators_rm(
    settings: &'static Vec<SettingsInd>
) -> MAP_INDICATORS_RM {
    settings
        .iter()
        .map(
            |v| (
            v.key.as_str(),
            match v.key.as_str() {
                "sma" => |src: &SrcEl, args: &Vec<T_ARGS>, rm: &mut Vec<T_HASHMAP>| {
                    let rm = rm.first_mut().expect("sma_rm not found");
                    match rm {
                        T_HASHMAP::VecF64(rm) => sma_rm(
                            src.open,
                            args[0].unwrap_usize(),
                            rm,
                        ),
                        _ => panic!("rm type not found.")
                    }
                },
                "ema" => |src: &SrcEl, _: &Vec<T_ARGS>, rm: &mut Vec<T_HASHMAP>| {
                    let rm = rm.get_mut(0).expect("rm ema not found");
                    match rm {
                        T_HASHMAP::Float64(rm) => ema_rm(
                            src.open,
                            rm,
                        ),
                        _ => panic!("rm type not found."),
                    }
                },
                "rsi" => |src: &SrcEl, _: &Vec<T_ARGS>, rm: &mut Vec<T_HASHMAP>| {
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
                            _ => panic!("rm type not found."),
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
                        panic!("rm type not found.")
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
                            _ => panic!("rm type not found."),
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
                        panic!("rm type not found.")
                    }
                },
                _ => panic!("key indication unknown"),
            })
        )
        .collect()
}


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::implicit_hasher)]
#[allow(clippy::ptr_arg)]
pub fn indications_gw_rm(
    src: &SrcEl,
    settings: &'static Vec<SettingsInd>,
    rm: &mut FxHashMap<&'static str, Vec<T_HASHMAP>>,
    map_args_rm: &MAP_ARGS_RM,
    map_indicators_rm: &MAP_INDICATORS_RM
) -> FxHashMap<&'static str, f64>
{ 
    settings
        .iter()
        .map(
            |v| 
            {
                let key_uniq_str = v.key_uniq.as_str();
                (
                    key_uniq_str,
                    map_indicators_rm
                        .get(v.key.as_str())
                        .expect("indicators not found in map indicators")(
                            src, 
                            map_args_rm.get(key_uniq_str).expect("map args rm not initializeted"),
                            rm.get_mut(key_uniq_str).expect("rm not found"),
                        )
                )
            }
        )
        .collect()
}