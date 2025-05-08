use std::borrow::Borrow;

use bc_utils_lg::types::structures::*;
use bc_utils_lg::types::maps::MAP;
use num_traits::Float;


pub fn bf_trend_ma<T, V>(
    src: &SRC_ARG<V>,
    exc_last: &bool,
) -> MAP<&'static str, T>
where 
    T: Float,
    V: Borrow<T>,
{
    let sub_ = usize::from(*exc_last);
    let src_new = &src[src.len() - 2 - sub_..src.len() - sub_];
    let mut res = MAP::default();
    let first = src_new[0].borrow();
    let secnd = src_new[1].borrow();
    if secnd > first {
        res.insert("res", T::one());
    } else {
        res.insert("res", T::zero());
    }
    res.insert("src", *src.last().unwrap().borrow());
    res
    
}

pub fn bf_trend_ma_abstr<T, V>(
    src: &SRCS_ARG<V>,
    _: &ARGS<T>,
    exc_last: &bool,
) -> MAP<&'static str, T>
where 
    T: Float,
    V: Borrow<T>,
{
    bf_trend_ma(src[0], exc_last)
}