#![allow(non_camel_case_types)]

use rustc_hash::FxHashMap;

use crate::enums::indicators::*;


// pub type COLL<T> = Vec<T>;
pub type BF_VEC<T> = Vec<T_HASHMAP<T>>;
pub type ARGS<T> = Vec<T_ARGS<T>>;
pub type SRC<T> = FxHashMap<String, T>;
pub type SRCS<'a, T> = FxHashMap<String, Vec<T>>;
pub type SRC_ARG<T> = [T];
pub type SRCS_ARG<'a, T> = [&'a SRC_ARG<T>];
pub type SRCS1_ARG<'a, T> = [&'a SRCS_ARG<'a, T>];