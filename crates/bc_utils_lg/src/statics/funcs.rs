use num_traits::Float;

use crate::types::structures::*;

pub fn fn_ind_bf_abstr_default<T>(
    _: &SRCS_ARG<T>,
    _: &ARGS<T>,
    _: &bool,
) -> BF_VEC<T>
where 
    T: Float
{
    BF_VEC::default()
}