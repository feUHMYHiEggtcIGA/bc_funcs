use num_traits::Float;

use crate::types::structures_abstr::*;

pub fn fn_bf_mod_abstr_default<T>(
    _: &SRC_ARG<T>,
    _: &SRCS_ARG<T>,
    _: &ARGS<T>,
    _: &bool,
) -> BF_VEC<T>
where 
    T: Float
{
    BF_VEC::default()
}