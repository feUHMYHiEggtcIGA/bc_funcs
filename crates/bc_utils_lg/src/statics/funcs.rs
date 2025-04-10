use num_traits::Float;

use crate::types::structures_abstr::*;

pub fn fn_bf_mod_abstr_default<'a, T>(
    _: &SRC_EL_ABSTR<T>,
    _: &Vec<&Vec<T>>, 
    _: &ARGS<T>,
    _: &bool,
) -> BF_VEC<'a, T>
where 
    T: Float
{
    BF_VEC::default()
}