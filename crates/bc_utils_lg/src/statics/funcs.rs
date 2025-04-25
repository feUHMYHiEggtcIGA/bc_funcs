use num_traits::Float;

use crate::structs_and_types::structures_abstr::*;

pub fn fn_bf_mod_abstr_default<T, U>(
    _: &SRC_ARG<T>,
    _: &SRCS_ARG<T>,
    _: &ARGS<T, U>,
    _: &bool,
) -> BF_VEC<T>
where 
    T: Float
{
    BF_VEC::default()
}