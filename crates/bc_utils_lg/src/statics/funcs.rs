use crate::types::indicators::*;
use num_traits::Float;

use crate::types::indicators::RM_VEC;

pub fn func_rm_mod_abstr_df<'a, T>(
    _: &[T],
    _: &Vec<&[T]>, 
    _: &ARGS<T>,
    _: &bool,
) -> RM_VEC<'a, T>
where 
    T: Float
{
    RM_VEC::default()
}