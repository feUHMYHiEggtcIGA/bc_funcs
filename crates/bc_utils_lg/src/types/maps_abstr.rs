#![allow(non_camel_case_types)]

use rustc_hash::FxHashMap;

use crate::types::funcs_abstr::*;
use crate::types::structures_abstr::*;


pub type MAP_BF_VEC<'a, T> = FxHashMap<&'static str, BF_VEC<T>>;
pub type MAP1_BF_VEC<'a, T> = FxHashMap<&'static str, MAP_BF_VEC<'a, T>>;
pub type MAP_IND_T<T> = FxHashMap<&'static str, IND_T<T>>;
pub type MAP_FUNC_T<T> = FxHashMap<&'static str, FUNC_T<T>>;
pub type MAP_IND_COLL<C, T> = FxHashMap<&'static str, IND_COLL<C, T>>;
pub type MAP_IND_T_BF<T> = FxHashMap<&'static str, IND_T_BF<T>>;
pub type MAP_FUNC_BF_MOD<'a, T> = FxHashMap<&'static str, FUNC_BF_MOD<'a, T>>;
pub type MAP_FUNC_BF_IND<'a, T> = FxHashMap<&'static str, FUNC_BF_IND<'a, T>>;
pub type MAP_ARGS<T> = FxHashMap<&'static str, ARGS<T>>;
pub type MAP1_ARGS<T> = FxHashMap<&'static str, MAP_ARGS<T>>;
pub type MAP2_ARGS<T> = FxHashMap<&'static str, MAP1_ARGS<T>>;
pub type MAP_MOD_T_BF<T> = FxHashMap<&'static str, MOD_T_BF<T>>;
pub type MAP_MOD_T<T> = FxHashMap<&'static str, MOD_T<T>>;
pub type MAP_MOD_T_FROM_COLL<T> = FxHashMap<&'static str, MOD_T_FROM_COLL<T>>;
pub type MAP_MOD_COLL<C, T> = FxHashMap<&'static str, MOD_COLL<C, T>>;
pub type MAP_FUNC_USIZE<T> = FxHashMap<&'static str, FUNC_USIZE<T>>;
pub type MAP_USIZE = FxHashMap<&'static str, usize>;
pub type MAP_VEC<'a, T> = FxHashMap<&'static str, Vec<T>>;