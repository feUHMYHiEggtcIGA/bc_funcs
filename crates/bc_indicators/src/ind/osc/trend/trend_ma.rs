use bc_utils_lg::types::structures::*;
use bc_utils_lg::types::maps::{MAP, MAP_BF_VEC};
use num_traits::Float;

use crate::bf::trend_ma::bf_trend_ma;


pub fn trend_ma_bf<T>(
    src: &T,
    bf: &mut MAP<&'static str, T>
) -> T
where 
    T: Float
{
    let v = &bf["src"];
    let mut res = bf["res"];
    if src > v {
        res = T::one();
        bf.insert("res", res);
    } else if src < v{
        res = T::zero();
        bf.insert("res",res);
    }
    bf.insert("src", *src);
    res
}

pub fn trend_ma_bf_abstr<T>(
    src: &SRC_ARG<T>,
    _: &ARGS<T>, 
    bf: & mut BF_VEC<T>
) -> T
where 
    T: Float
{
    trend_ma_bf(&src[0], bf.first_mut().unwrap().unwrap_f())
}

pub fn trend_ma_coll<C, T>(
    src: &SRC_ARG<T>,
) -> C
where 
    T: Float,
    C: FromIterator<T>,
{
    let mut bf = bf_trend_ma(&src[..2], &false);
    src
        [2..]
        .iter()
        .map(|v| trend_ma_bf(v, &mut bf))
        .collect()
}

pub fn trend_ma_coll_abstr<C, T>(
    src: &SRC_ARG<T>,
    _: &ARGS<T>,
) -> C
where 
    T: Float,
    C: FromIterator<T>,
{
    trend_ma_coll(src)
}