#![allow(non_camel_case_types)]

use crate::types::structures::*;

pub type FUNC_USIZE<T> = fn(&ARGS<T>) -> usize;
pub type FUNC_T<T> = fn(&ARGS<T>) -> T;
pub type IND_T_BF<T> = fn(&SRC_ARG<T>, &ARGS<T>, &mut BF_VEC<T>) -> T;
pub type IND_T<T> = fn(&SRCS_ARG<T>, &ARGS<T>) -> T;
pub type MOD_T<T> = fn(&T, &SRC_ARG<T>, &ARGS<T>) -> T;
pub type MOD_T_FROM_COLL<T> = fn(&SRC_ARG<T>, &SRCS_ARG<T>, &ARGS<T>) -> T;
pub type MOD_T_BF<T> = fn(&T, &SRC_ARG<T>, &ARGS<T>, &mut BF_VEC<T>) -> T;
pub type FUNC_BF_IND<'a, T> = fn(&SRCS_ARG<T>, &ARGS<T>, &bool) -> BF_VEC<T>;
pub type FUNC_BF_MOD<'a, T> = fn(&SRC_ARG<T>, &SRCS_ARG<T>, &ARGS<T>, &bool) -> BF_VEC<T>;
pub type IND_COLL<C, T> = fn(&SRCS_ARG<T>, &ARGS<T>) -> C;
pub type MOD_COLL<C, T> = fn(&SRC_ARG<T>, &SRCS_ARG<T>, &ARGS<T>) -> C;