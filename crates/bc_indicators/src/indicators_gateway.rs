use std::iter::Sum;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::structs::src::SRC_EL;
use bc_utils_lg::structs::settings::SETTINGS_IND;
#[allow(clippy::wildcard_imports)]
use bc_utils_lg::enums::indicators::*;
#[allow(clippy::wildcard_imports)]
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
pub fn map_args_rm<T>(
    settings: &'static Vec<SETTINGS_IND>
) -> MAP_ARGS_RM<T>
where  
    T: Float,
    std::collections::HashMap<&'static str, std::vec::Vec<bc_utils_lg::enums::indicators::T_ARGS<T>>, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>: std::iter::FromIterator<(&'static str, std::vec::Vec<bc_utils_lg::enums::indicators::T_ARGS<T>>)>
{
    settings
        .iter()
        .map(
            |setting| 
            (
                setting.key_uniq.as_str(),
                match setting.key.as_str() {
                    "sma" => vec![T_ARGS::<T>::Usize(setting.kwargs_usize["window"])],
                    "rma" | "ema" | "rsi" => vec![T_ARGS::<T>::None(())],
                    "tqo_b" => vec![
                        T_ARGS::<T>::Usize(setting.kwargs_usize["window_noise"]),
                        T_ARGS::<T>::Float(T::from(setting.kwargs_f64["correlation_factor"]).unwrap()),
                        T_ARGS::<T>::String(setting.kwargs_string["noise_type"].clone()),
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
pub fn map_indicators_rm<T>(
    settings: &'static Vec<SETTINGS_IND>
) -> MAP_INDICATORS_RM<T>
where  
    T: Float,
    T: Sum,
{
    settings
        .iter()
        .map(
            |v| (
            v.key.as_str(),
            match v.key.as_str() {
                "sma" => |src: &SRC_EL<T>, args: &Vec<T_ARGS<T>>, rm: &mut Vec<T_HASHMAP<T>>| {
                    let rm = rm.first_mut().expect("sma_rm not found");
                    match rm {
                        T_HASHMAP::VecF(rm) => sma_rm(
                            src.open,
                            args[0].unwrap_usize(),
                            rm,
                        ),
                        _ => panic!("rm type not found.")
                    }
                },
                "ema" => |src: &SRC_EL<T>, _: &Vec<T_ARGS<T>>, rm: &mut Vec<T_HASHMAP<T>>| {
                    let rm = rm.get_mut(0).expect("rm ema not found");
                    match rm {
                        T_HASHMAP::Float(rm) => ema_rm(
                            src.open,
                            rm,
                        ),
                        _ => panic!("rm type not found."),
                    }
                },
                "rsi" => |src: &SRC_EL<T>, _: &Vec<T_ARGS<T>>, rm: &mut Vec<T_HASHMAP<T>>| {
                    let mut v1: Option<&mut FxHashMap<&'static str, T>> = None;
                    let mut v2: Option<&mut FxHashMap<&'static str, T>> = None;
                    let mut v3: Option<&mut FxHashMap<&'static str, T>> = None;
                    for (i, map) in rm
                        .iter_mut()
                        .enumerate() {
                        match (i, map) {
                            (0, T_HASHMAP::Float(map)) => v1 = Some(map),
                            (1, T_HASHMAP::Float(map)) => v2 = Some(map),
                            (2, T_HASHMAP::Float(map)) => v3 = Some(map),
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
                "tqo_b" => |src: &SRC_EL<T>, v: &Vec<T_ARGS<T>>, rm:  &mut Vec<T_HASHMAP<T>>| {
                    let (
                        v1, 
                        v2,
                        v3,
                    ) = (v[0].unwrap_usize(), v[1].unwrap_f(), v[2].unwrap_string());
                    let mut v4: Option<&mut FxHashMap<&'static str, T>> = None;
                    let mut v5: Option<&mut FxHashMap<&'static str, T>> = None;
                    let mut v6: Option<&mut FxHashMap<&'static str, T>> = None;
                    let mut v7: Option<&mut FxHashMap<&'static str, Vec<T>>> = None;
                    for (i, map) in rm
                        .iter_mut()
                        .enumerate() {
                        match (i, map) {
                            (0, T_HASHMAP::Float(map)) => v4 = Some(map),
                            (1, T_HASHMAP::Float(map)) => v5 = Some(map),
                            (2, T_HASHMAP::Float(map)) => v6 = Some(map),
                            (3, T_HASHMAP::VecF(map)) => v7 = Some(map),
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
pub fn indicators_gw_rm<T>(
    src: &SRC_EL<T>,
    settings: &'static Vec<SETTINGS_IND>,
    rm: &mut FxHashMap<&'static str, Vec<T_HASHMAP<T>>>,
    map_args_rm: &MAP_ARGS_RM<T>,
    map_indicators_rm: &MAP_INDICATORS_RM<T>,
) -> FxHashMap<&'static str, T>
where 
    T: Float,
    T: Sum,
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

// #[allow(clippy::ptr_arg)]
// pub fn sma_rm_abstr<'c, T>(
//     src: &SRC_EL<T>, 
//     args: &Vec<T_ARGS<T>>, 
//     rm: &'c mut Vec<T_HASHMAP<'c, T>>
// ) -> T
// where 
//     T: Float,
//     T: Sum,
// {
//     sma_rm(
//         src.open,
//         args.first().expect("first arg not found").unwrap_usize(),
//         rm.first_mut().expect("rm sma not found").unwrap_vec_f(),
//     )
// }

// #[allow(clippy::needless_borrows_for_generic_args)]
// pub fn ema_rm_abstr<'c, T>(
//     src: &SRC_EL<T>, 
//     _: &Vec<T_ARGS<T>>, 
//     rm: &'c mut Vec<T_HASHMAP<'c, T>>
// ) -> T 
// where 
//     T: Float,
// {   
//     ema_rm(
//         &src.open,
//         rm.first_mut().expect("rm ema not found").unwrap_f(),
//     )
// }

// #[allow(clippy::needless_borrows_for_generic_args)]
// pub fn rma_rm_abstr<'c, T>(
//     src: &SRC_EL<T>, 
//     _: &Vec<T_ARGS<T>>, 
//     rm: &'c mut Vec<T_HASHMAP<'c, T>>
// ) -> T 
// where 
//     T: Float,
// {
//     rma_rm(
//         &src.open,
//         rm.first_mut().expect("rm rma not found").unwrap_f(),
//     )
// }

// #[allow(clippy::pedantic)]
// pub fn rsi_rm_abstr<'a, T>(
//     src: &SRC_EL<T>,
//     _: &Vec<T_ARGS<T>>, 
//     rm: & mut Vec<T_HASHMAP<T>>
// ) -> T 
// where 
//     T: Float,
// {
//     let (rm1, rma) = rm.split_at_mut(1);
//     let (rma1, rma2) = rma.split_at_mut(1);

//     rsi_rm(
//         &src.open,
//         rm1.last_mut().expect("rm rsi not found").unwrap_f(),
//         rma1.last_mut().expect("rm rma1 in rm rsi not found").unwrap_f(),
//         rma2.last_mut().expect("rm rma2 in rm rsi not found").unwrap_f(),
//     )
// }

// #[allow(clippy::pedantic)]
// #[allow(clippy::ptr_arg)]
// pub fn tqo_b_rm_abstr<'a, T>(
//     src: &SRC_EL<T>,
//     args: &Vec<bc_utils_lg::enums::indicators::T_ARGS<T>>,
//     rm: &'a mut Vec<T_HASHMAP<'a, T>>
// ) -> T
// where 
//     T: Float,
//     T: Sum,
// {
//     let (rm1, rm_ema) = rm.split_at_mut(1);
//     let (rm_ema_fast, rm_ema_sma) = rm_ema.split_at_mut(1);
//     let (rm_ema_slow, rm_sma,) = rm_ema_sma.split_at_mut(1);

//     tqo_b_rm(
//         &src.open,
//         args.first().expect("arg 1 tqo_b_rm not found").unwrap_usize(),
//         args.get(1).expect("arg 2 tqo_b_rm not found").unwrap_f(),
//         args.get(2).expect("arg 3 tqo_b_rm not found").unwrap_string(),
//         rm1.last_mut().expect("rm rsi not found").unwrap_f(),
//         rm_ema_fast.last_mut().expect("rm rma1 in rm rsi not found").unwrap_f(),
//         rm_ema_slow.last_mut().expect("rm rma2 in rm rsi not found").unwrap_f(),
//         rm_sma.last_mut().expect("rm sma in tqo_b_rm not found").unwrap_vec_f(),
//     )
// }

// pub fn map_indicators_rm() -> MAP_INDICATORS_RM
// where 
//     T: Float,
//     T: Sum,
// {
//     FxHashMap::from_iter([
//         // ("sma", sma_rm_abstr as fn(&SRC_EL<T>, &Vec<T_ARGS<T>>,  &mut Vec<T_HASHMAP<T>>)),
//         // ("ema", ema_rm_abstr as fn(&SRC_EL<T>, &Vec<T_ARGS<T>>,  &'a mut Vec<T_HASHMAP<'a, T>>) -> T),
//         // ("rma", rma_rm_abstr as fn(&SRC_EL<T>, &Vec<T_ARGS<T>>,  &'a mut Vec<T_HASHMAP<'a, T>>) -> T),
//         // ("rsi", rsi_rm_abstr as fn(&SRC_EL<T>, &Vec<T_ARGS<T>>,  &'a mut Vec<T_HASHMAP<'a, T>>) -> T),
//         ("tqo_b", tqo_b_rm_abstr as fn(&SRC_EL<T>, &Vec<T_ARGS<T>>,  &'a mut Vec<T_HASHMAP<'a, T>>) -> T),
//     ])
// }