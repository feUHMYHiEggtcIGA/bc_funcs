#![allow(non_camel_case_types)]

use rustc_hash::FxHashMap;

use crate::structs_and_types::funcs_abstr::*;
use crate::structs_and_types::structures_abstr::*;


pub type MAP<K, V> = FxHashMap<K, V>;
pub type MAP_BF_VEC<'a, T> = FxHashMap<&'static str, BF_VEC<T>>;
pub type MAP1_BF_VEC<'a, T> = FxHashMap<&'static str, MAP_BF_VEC<'a, T>>;
pub type MAP_IND_T<F> = FxHashMap<&'static str, IND_T<F>>;
pub type MAP_FUNC_T<F> = FxHashMap<&'static str, FUNC_T<F>>;
pub type MAP_IND_COLL<C, F> = FxHashMap<&'static str, IND_COLL<C, F>>;
pub type MAP_IND_T_BF<F> = FxHashMap<&'static str, IND_T_BF<F>>;
pub type MAP_FUNC_BF_MOD<'a, F> = FxHashMap<&'static str, FUNC_BF_MOD<'a, F>>;
pub type MAP_FUNC_BF_IND<'a, F> = FxHashMap<&'static str, FUNC_BF_IND<'a, F>>;
pub type MAP_ARGS<'a, F> = FxHashMap<&'static str, ARGS<'a, F>>;
pub type MAP1_ARGS<'a, F> = FxHashMap<&'static str, MAP_ARGS<'a, F>>;
pub type MAP2_ARGS<'a, F> = FxHashMap<&'static str, MAP1_ARGS<'a, F>>;
pub type MAP_MOD_T_BF<F> = FxHashMap<&'static str, MOD_T_BF<F>>;
pub type MAP_MOD_T<F> = FxHashMap<&'static str, MOD_T<F>>;
pub type MAP_MOD_T_FROM_COLL<F> = FxHashMap<&'static str, MOD_T_FROM_COLL<F>>;
pub type MAP_MOD_COLL<C, F> = FxHashMap<&'static str, MOD_COLL<C, F>>;
pub type MAP_FUNC_USIZE<F> = FxHashMap<&'static str, FUNC_USIZE<F>>;
pub type MAP_USIZE = FxHashMap<&'static str, usize>;
pub type MAP_COLL<'a, C> = FxHashMap<&'static str, C>;