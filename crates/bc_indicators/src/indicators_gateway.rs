// use std::collections::HashMap;
// use std::sync::LazyLock;

// use crate::indicators::no_oscillators::trend::*;
// use crate::indicators::no_oscillators::other::*;
// use crate::indicators::oscillators::multipliers::*;
// use crate::indicators::oscillators::trend::*;
// use crate::indicators::oscillators::other::*;

// use bc_utils_lg::structs::{
//     SrcEl,
//     SettingsInd,
// };

// type FuncType = fn(&str) -> &str;

// static IND_CHOICES_RM: LazyLock<HashMap<&str, fn()>> = LazyLock::new(|| {
//     let mut map = HashMap::new();
//     map.insert("rsi", rsi_rm as fn());
//     map.insert("ema", ema_rm as fn());
//     map
// });

// pub fn indications_gw(
//     src: &SrcEl,
//     settings: &SettingsInd,
// ) {

// }

// pub fn indications_gw_ft(
//     src: &SrcEl,
//     settings: i8,
// ) {}

// pub fn indications_gw_coll(
//     src: &SrcEl,
//     settings: i8,
// ) {}