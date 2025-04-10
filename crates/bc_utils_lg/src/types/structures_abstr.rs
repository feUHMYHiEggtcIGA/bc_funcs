#![allow(non_camel_case_types)]

use rustc_hash::FxHashMap;

use crate::enums::indicators::*;


pub type BF_VEC<'a, T> = Vec<T_HASHMAP<'a, T>>;
pub type ARGS<T> = Vec<T_ARGS<T>>;
pub type SRC_EL<T> = FxHashMap<String, T>;
pub type SRC_EL_ABSTR<T> = Vec<T>;
pub type SRC<'a, T> = FxHashMap<String, Vec<T>>;
pub type SRC_ABSTR<T> = Vec<Vec<T>>;
pub type SRC_ARG<T> = [T];
pub type VEC_LT<'a, T> = Vec<&'a T>;
pub type VEC_LSRC_ARG<'a, T> = Vec<&'a SRC_ARG<T>>;