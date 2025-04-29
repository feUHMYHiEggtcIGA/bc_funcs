use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::types::structures::*;
use bc_utils_lg::enums::indicators::*;


#[allow(clippy::pedantic)]
pub fn bf_nohesi<T, V>(
    src: &SRC_ARG<V>,
    hesi: &T,
    exc_last: &bool,
) -> FxHashMap<&'static str, T>
where 
    T: Float,
    V: Borrow<T>,
{
    let mut peak_l = T::zero();
    let mut btm_l= T::zero();
    let len = src.len();
    let mut res = T::nan();

    for el in {
        if *exc_last {src[len - 2 - 1..len - 1].iter()}
        else {src[len - 2..].iter()}
    } {
        let el_brwd = el.borrow();
        let hesit = *el_brwd * *hesi;
        let (peak, btm);
        if *el_brwd > peak_l {
            peak = *el_brwd;
            btm = *el_brwd - hesit;
        } else if *el_brwd < btm_l {
            peak = *el_brwd + hesit;
            btm = *el_brwd;
        } else {
            peak = peak_l;
            btm = btm_l;
        }
        if btm < btm_l || peak > peak_l {
            res = *el_brwd;
        }
        peak_l = peak;
        btm_l = btm;
    }
    FxHashMap::from_iter([
        ("peak", peak_l),
        ("btm", btm_l),
        ("res", res)
    ])
}

pub fn bf_nohesi_abstr<'a, T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>,
    exc_last: &bool
) -> BF_VEC<T>
where  
    T: Float,
{
    vec![T_HASHMAP::Float(bf_nohesi(src[0], args.first().unwrap().unwrap_f(), exc_last))]
}