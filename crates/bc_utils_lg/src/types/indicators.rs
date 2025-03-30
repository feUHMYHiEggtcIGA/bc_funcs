use rustc_hash::FxHashMap;

use crate::enums::indicators::{
    T_ARGS,
    T_HASHMAP,
};
use crate::structs::src::SRC_EL;


#[allow(non_camel_case_types)]
pub type MAP_ARGS_RM<T> = FxHashMap<&'static str, Vec<T_ARGS<T>>>;
#[allow(non_camel_case_types)]
pub type MAP_INDICATORS_RM<T> = FxHashMap<&'static str, fn(&SRC_EL<T>, &Vec<T_ARGS<T>>,  &mut Vec<T_HASHMAP<T>>) -> T>;
// #[allow(non_camel_case_types)]
// pub type MAP_MODS<'a, T>
//     = FxHashMap<&'static str, fn(&Vec<T_ARGS>, Vec<T>) -> T>;
// #[allow(non_camel_case_types)]
// pub type MAP_MODS_RM<'a, T> 
//     = FxHashMap<&'a str, fn(&Vec<T_ARGS>, Vec<T>, &mut Vec<T_HASHMAP<'a>>) -> T>;