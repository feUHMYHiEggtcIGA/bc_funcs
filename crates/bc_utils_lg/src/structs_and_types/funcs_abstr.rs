#![allow(non_camel_case_types)]

use crate::structs_and_types::structures_abstr::*;

pub type FUNC_USIZE<F, T> = fn(&ARGS<F, T>) -> usize;
pub type FUNC_T<F, T> = fn(&ARGS<F, T>) -> T;
pub type IND_T_BF<F, T> = fn(&SRC_ARG<T>, &ARGS<F, T>, &mut BF_VEC<T>) -> T;
pub type IND_T<F, T> = fn(&SRCS_ARG<T>, &ARGS<F, T>) -> T;
pub type MOD_T<F, T> = fn(&T, &SRC_ARG<T>, &ARGS<F, T>) -> T;
pub type MOD_T_FROM_COLL<F, T> = fn(&SRC_ARG<T>, &SRCS_ARG<T>, &ARGS<F, T>) -> T;
pub type MOD_T_BF<F, T> = fn(&T, &SRC_ARG<T>, &ARGS<F, T>, &mut BF_VEC<T>) -> T;
pub type FUNC_BF_IND<'a, F, T> = fn(&SRCS_ARG<T>, &ARGS<F, T>, &bool) -> BF_VEC<T>;
pub type FUNC_BF_MOD<'a, F, T> = fn(&SRC_ARG<T>, &SRCS_ARG<T>, &ARGS<F, T>, &bool) -> BF_VEC<T>;
pub type IND_COLL<C, F, T> = fn(&SRCS_ARG<T>, &ARGS<F, T>) -> C;
pub type MOD_COLL<C, F, T> = fn(&SRC_ARG<T>, &SRCS_ARG<T>, &ARGS<F, T>) -> C;