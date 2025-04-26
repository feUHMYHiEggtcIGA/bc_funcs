use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::types::structures::*;
use bc_utils_lg::enums::indicators::*;
use bc_utils::nums::sign;

use crate::bf::ema::bf_ema;
use crate::ind::osc::trend::tqo_b::*;
use crate::ind::no_osc::trend::ema::{alpha_ema, ema_bf};


#[allow(clippy::missing_panics_doc)]
pub fn bf_tqo_b<'a, T, V>(
    src: &SRC_ARG<V>,
    window_ema_fast: &usize,
    window_ema_slow: &usize,
    window_trend: &usize,
    window_noise: &usize,
    add_iters: &usize,
    noise_type: &str,
    exc_last: &bool,
) -> (
    FxHashMap<&'static str, T>,
    FxHashMap<&'static str, T>,
    FxHashMap<&'static str, T>,
    FxHashMap<&'static str, Vec<T>>,
)
where 
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    V: Borrow<T>,
{
    let len_src = if *exc_last {src.len() - 1} else {src.len()};
    let alpha_trend = alpha_ema(&T::from(*window_trend).unwrap());
    let num_need = window_tqo_b(window_ema_fast, window_ema_fast, window_trend, window_noise, add_iters);
    let src = &src[..len_src];
    let src_take = &src[..=(len_src - num_need + 1)];
    // todo: change exc_last => false
    let mut bf_ema_fast = bf_ema(src_take, window_ema_fast, &true,);
    let mut bf_ema_slow = bf_ema(src_take, window_ema_slow, &true,);
    let (mut reversal, mut reversal_l) = (T::zero(), T::nan());
    let (mut cpc, mut cpc_l) = (T::zero(), T::nan());
    let (mut trend, mut trend_l) = (T::nan(), T::nan());
    let mut diff = Vec::with_capacity(*window_noise);
    let mut src_l = src[..len_src - num_need]
        .last()
        .unwrap()
        .borrow();

    for (i, el) in src
        .iter()
        .skip(len_src - num_need)
        .enumerate()
    {
        let el_brwd = el.borrow();
        let ema_fast = ema_bf(el_brwd, &mut bf_ema_fast);
        let ema_slow = ema_bf(el_brwd, &mut bf_ema_slow);
        reversal = sign(ema_fast - ema_slow);
        if reversal == reversal_l {
            cpc = cpc_l + *el_brwd - *src_l;
            trend = trend_l * (T::one() - alpha_trend) + cpc * alpha_trend;
        } else {
            cpc = T::zero();
            trend = T::zero();
        }
        if i > num_need - *window_noise - 1 {
            diff.push(
                match noise_type {
                    "linear" => (cpc - trend).abs(),
                    _ => (cpc - trend).abs().powi(2)
                }
            );
        }
        reversal_l = reversal;
        cpc_l = cpc;
        trend_l = trend;
        src_l = el.borrow();
    }
    (
        FxHashMap::from_iter([
            ("alpha", alpha_trend),
            ("cpc", cpc),
            ("src", *src_l),
            ("reversal", reversal),
            ("trend", trend),
        ]),
        bf_ema_fast,
        bf_ema_slow,
        FxHashMap::from_iter([
            ("src", diff)
        ])
    )
}

pub fn bf_tqo_b_abstr<'a, T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>,
    exc_last: &bool,
) -> BF_VEC<T>
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    let bf = bf_tqo_b(
        src.first().unwrap(), 
        args.get(0).unwrap().unwrap_usize(),
        args.get(1).unwrap().unwrap_usize(),
        args.get(2).unwrap().unwrap_usize(),
        args.get(3).unwrap().unwrap_usize(),
        args.get(4).unwrap().unwrap_usize(),
        args.get(5).unwrap().unwrap_string(),
        exc_last,
    );
    vec![
        T_HASHMAP::Float(bf.0),
        T_HASHMAP::Float(bf.1),
        T_HASHMAP::Float(bf.2),
        T_HASHMAP::VecF(bf.3),
    ]
}