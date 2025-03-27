use rustc_hash::FxHashMap;

use crate::enums::indicators::{
    T_ARGS,
    T_HASHMAP,
};
use crate::structs::src::SrcEl;


#[allow(non_camel_case_types)]
pub type MAP_ARGS_RM = FxHashMap<&'static str, Vec<T_ARGS>>;
#[allow(non_camel_case_types)]
pub type MAP_INDICATORS_RM = FxHashMap<&'static str, fn(&SrcEl, &Vec<T_ARGS>,  &mut Vec<T_HASHMAP>) -> f64>;