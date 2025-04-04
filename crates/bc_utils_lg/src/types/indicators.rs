#![allow(non_camel_case_types)]

use rustc_hash::FxHashMap;

use crate::enums::indicators::{
    T_ARGS,
    T_HASHMAP,
};
use crate::structs::src::{
    SRC_EL,
    SRC,
};


pub type RM_VEC<'a, T> = Vec<T_HASHMAP<'a, T>>;
pub type ARGS<T> = Vec<T_ARGS<T>>;
pub type FUNC_aT_aABSTR<T> = fn(&ARGS<T>) -> T;
pub type FUNC_aUSIZE_aABSTR<T> = fn(&ARGS<T>) -> usize;
pub type IND_T_RM_aABSTR<T> = fn(&SRC_EL<T>, &ARGS<T>, &mut RM_VEC<T>) -> T;
pub type IND_T_aABSTR<T> = fn(&SRC<T>, &ARGS<T>) -> T;
pub type IND_COLL_aABSTR<C, T> = fn(&SRC<T>, &ARGS<T>) -> C;
pub type FUNC_RM_IND_aABSTR<'a, T> = fn(&SRC<T>, &ARGS<T>, &bool) -> RM_VEC<'a, T>;
pub type MOD_T_RM_aABSTR<T> = fn(&T, &Vec<&T>, &ARGS<T>, &mut RM_VEC<T>) -> T;
pub type MOD_T_aABSTR<T> = fn(&T, &Vec<&T>, &ARGS<T>) -> T;
pub type FUNC_RM_MOD_aABSTR<'a, T> = fn(&[T], &Vec<&[T]>, &ARGS<T>, &bool) -> RM_VEC<'a, T>;

pub type MAP_aRM_VEC<'a, T> = FxHashMap<&'static str, RM_VEC<'a, T>>;
pub type MAP_aMAP_RM_VEC<'a, T> = FxHashMap<&'static str, MAP_aRM_VEC<'a, T>>;
pub type MAP_aIND_T_ABSTR<T> = FxHashMap<&'static str, IND_T_aABSTR<T>>;
pub type MAP_aFUNC_T<T> = FxHashMap<&'static str, FUNC_aT_aABSTR<T>>;
pub type MAP_aIND_COLL_ABSTR<C, T> = FxHashMap<&'static str, IND_COLL_aABSTR<C, T>>;
pub type MAP_aIND_T_RM_ABSTR<T> = FxHashMap<&'static str, IND_T_RM_aABSTR<T>>;
pub type MAP_aFUNC_RM_MOD_ABSTR<'a, T> = FxHashMap<&'static str, FUNC_RM_MOD_aABSTR<'a, T>>;
pub type MAP_aFUNC_RM_IND_ABSTR<'a, T> = FxHashMap<&'static str, FUNC_RM_IND_aABSTR<'a, T>>;
pub type MAP_aARGS<T> = FxHashMap<&'static str, ARGS<T>>;
pub type MAP_aMAP_ARGS<T> = FxHashMap<&'static str, MAP_aARGS<T>>;
pub type MAP_aMOD_T_RM_ABSTR<T> = FxHashMap<&'static str, MOD_T_RM_aABSTR<T>>;
pub type MAP_aMOD_T_ABSTR<T> = FxHashMap<&'static str, MOD_T_aABSTR<T>>;
pub type MAP_aFUNC_T_ABSTR<T> = FxHashMap<&'static str, FUNC_aT_aABSTR<T>>;
pub type MAP_aFUNC_USIZE_ABSTR<T> = FxHashMap<&'static str, FUNC_aUSIZE_aABSTR<T>>;
pub type MAP_aUSIZE = FxHashMap<&'static str, usize>;
pub type MAP_aVEC<'a, T> = FxHashMap<&'static str, Vec<T>>;