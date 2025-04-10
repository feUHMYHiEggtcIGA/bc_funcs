#![allow(non_camel_case_types)]

use crate::types::structures_abstr::*;

pub type FUNC_T<T> = fn(&ARGS<T>) -> T;
pub type FUNC_USIZE<T> = fn(&ARGS<T>) -> usize;
pub type IND_T_BF<T> = fn(&SRC_EL<T>, &ARGS<T>, &mut BF_VEC<T>) -> T;
pub type IND_T<T> = fn(&SRC<T>, &ARGS<T>) -> T;
pub type IND_COLL<C, T> = fn(&SRC<T>, &ARGS<T>) -> C;
pub type FUNC_BF_IND<'a, T> = fn(&SRC<T>, &ARGS<T>, &bool) -> BF_VEC<'a, T>;
pub type MOD_T_BF<T> = fn(&T, &VEC_LT<T>, &ARGS<T>, &mut BF_VEC<T>) -> T;
pub type MOD_T<T> = fn(&T, &VEC_LT<T>, &ARGS<T>) -> T;
pub type MOD_COLL<C, T> = fn(&SRC_EL<T>, &VEC_LSRC_ARG<T>, &ARGS<T>) -> C;
pub type FUNC_BF_MOD<'a, T> = fn(&SRC_EL<T>, &Vec<&Vec<T>>, &ARGS<T>, &bool) -> BF_VEC<'a, T>;