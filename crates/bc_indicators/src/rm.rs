/// functions that return rm values
/// return ready-made values for reuse
/// in rm functions

use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils::transf;
use bc_utils::create;
use bc_utils_lg::types::indicators::*;
use bc_utils_lg::enums::indicators::*;
use bc_utils_lg::structs::src::SRC as SRC_STRC;

use super::indicators::no_oscillators::trend as trend_no_osc;
use super::indicators::oscillators::trend as trend_osc;


pub fn rm_trend_ma<T>() -> FxHashMap<&'static str, T> 
where 
    T: Float,
{
    FxHashMap::from_iter([
        ("trend", T::zero()),
        ("l", T::zero()),
    ])
}

// todo: check links of returned values
pub fn rm_sma<T>(
    src: &[T],
    window: &usize,
    exc_last: &bool,
) -> FxHashMap<&'static str, Vec<T>>
where 
    T: Float,
{
    let len = src.len();
    let mut src_new = src
        .iter()
        .skip(len - *window - 1)
        .map(Borrow::borrow)
        .collect::<Vec<&T>>();
    if *exc_last {
        transf::roll_slice1(&mut src_new, &1);
        FxHashMap::from_iter([
            ("src", src_new.into_iter().skip(1).copied().collect()),
        ])
    } else {
        FxHashMap::from_iter([
            ("src", src_new.into_iter().copied().collect()),
        ])
    }
}

pub fn rm_sma_abstr<'a, T>(
    src: &SRC_STRC< T>,
    args: &ARGS<T>,
    exc_last: &bool,
) -> RM_VEC<'a, T>
where 
    T: Float,
{
    vec![T_HASHMAP::VecF(rm_sma(src.open, args.first().unwrap().unwrap_usize(), exc_last))]
}

#[allow(clippy::missing_panics_doc)]
pub fn rm_ema<'a, T, V>(
    src: &[V],
    window: &usize,
    exc_last: &bool,
) -> FxHashMap<&'static str, T>
where 
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    V: Borrow<T>,
{
    let len = src.len();
    let mut res = T::zero();
    let window_t = T::from(*window).unwrap();
    
    let alpha = trend_no_osc::alpha_ema(&window_t);
    for (i, el) in {
        if *exc_last {&src[len - *window * 10 - 1..len - 1]}
        else {&src[len - *window * 10..len]}
    }
        .iter()
        .enumerate()
    {
        if i < *window {
            res += *el.borrow();
            continue;
        }
        if i == *window - 1 {
            res /= window_t;
        }
        res = trend_no_osc::ema(
            el.borrow(), 
            &res, 
            &alpha,
        );
    }
    FxHashMap::from_iter([
        ("alpha", alpha),
        ("res", res)
    ])
}

// todo: check links of returned values
pub fn rm_ema_abstr<'a, T>(
    src: &SRC_STRC< T>,
    args: &ARGS<T>,
    exc_last: &bool,
) -> RM_VEC<'a, T>
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    vec![T_HASHMAP::Float(rm_ema(src.open, args.first().unwrap().unwrap_usize(), exc_last))]
}

#[allow(clippy::missing_panics_doc)]
pub fn rm_rma<'a, T, V>(
    src: &[V],
    window: &usize,
    exc_last: &bool,
) -> FxHashMap<&'static str, T>
where 
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    V: Borrow<T>,
{
    let mut res = T::zero();
    let len = src.len();
    let window_t = T::from(*window).unwrap();
    
    let alpha = trend_no_osc::alpha_rma(&window_t);
    for (i, el) in {
        if *exc_last {&src[len - *window * 10 - 1..len - 1]}
        else {&src[len - *window * 10..len]}
    }
        .iter()
        .enumerate() 
    {
        if i < *window {
            res += *el.borrow();
            continue;
        }
        if i == *window - 1{
            res /= window_t;
        }
        res = trend_no_osc::rma(
            el.borrow(), 
            &res, 
            &alpha,
        );
    }
    FxHashMap::from_iter([
        ("alpha", alpha),
        ("res", res
        ),
    ])
}

pub fn rm_rma_abstr<'a, T>(
    src: &SRC_STRC< T>,
    args: &ARGS<T>,
    exc_last: &bool,
) -> RM_VEC<'a, T>
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    vec![T_HASHMAP::Float(rm_rma(src.open, args.first().unwrap().unwrap_usize(), exc_last))]
}

#[allow(clippy::implicit_hasher)]
#[allow(clippy::type_complexity)]
pub fn rm_rsi<'a, T, V>(
    src: &[V],
    window: &usize,
    exc_last: &bool,
) -> (
    FxHashMap<&'static str, T>,
    FxHashMap<&'static str, T>, 
    FxHashMap<&'static str, T>,
)
where 
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    V: Borrow<T>,
{
    let mut u: Vec<T> = Vec::new();
    let mut d: Vec<T> = Vec::new();
    let mut src_l = T::nan();
    let len_src = src.len();
    let w = *window * 10;

    for (i, el) in {
        if *exc_last {&src[len_src - *window * 10 - 2..len_src - 1]}
        else {&src[len_src - *window * 10 - 1..]}
    }
        .iter()
        .enumerate()
    {
        if i == 0 {
            src_l = *el.borrow();
            continue;
        }
        let change = *el.borrow() - src_l;
        u.push(change.max(T::zero()));
        d.push((-change).max(T::zero()));
        src_l = *el.borrow();
    }
    (
        FxHashMap::from_iter([("src", src_l)]),
        rm_rma(u.as_slice(), window, &false),
        rm_rma(d.as_slice(), window, &false)
    )
}

pub fn rm_rsi_abstr<'a, T>(
    src: &SRC_STRC<T>,
    args: &ARGS<T>,
    exc_last: &bool,
) -> RM_VEC<'a, T>
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    let rm = rm_rsi(src.open, args.first().unwrap().unwrap_usize(), exc_last);
    vec![
        T_HASHMAP::Float(rm.0),
        T_HASHMAP::Float(rm.1),
        T_HASHMAP::Float(rm.2),
    ]
}

#[allow(clippy::missing_panics_doc)]
pub fn rm_tqo_b<'a, T, V>(
    src: &[V],
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
    // test this
    let len_src = if *exc_last {src.len() - 1} else {src.len()};
    let alpha_trend = trend_no_osc::alpha_ema(&T::from(*window_trend).unwrap());
    let num_need = trend_osc::window_tqo_b(window_ema_fast, window_ema_fast, window_trend, window_noise, add_iters);
    let src = &src[..len_src];
    let src_take = &src[..=(len_src - num_need + 1)];
    // todo: change exc_last => false
    let mut rm_ema_fast = rm_ema(src_take, window_ema_fast, &true);
    let mut rm_ema_slow = rm_ema(src_take, window_ema_slow, &true);
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
        let ema_fast = trend_no_osc::ema_rm(el_brwd, &mut rm_ema_fast);
        let ema_slow = trend_no_osc::ema_rm(el_brwd, &mut rm_ema_slow);
        reversal = create::sign(ema_fast - ema_slow);
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
        rm_ema_fast,
        rm_ema_slow,
        FxHashMap::from_iter([
            ("src", diff)
        ])
    )
}

pub fn rm_tqo_b_abstr<'a, T>(
    src: &SRC_STRC<T>,
    args: &ARGS<T>,
    exc_last: &bool,
) -> RM_VEC<'a, T>
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    let rm = rm_tqo_b(
        src.open, 
        args.get(0).unwrap().unwrap_usize(),
        args.get(1).unwrap().unwrap_usize(),
        args.get(2).unwrap().unwrap_usize(),
        args.get(3).unwrap().unwrap_usize(),
        args.get(4).unwrap().unwrap_usize(),
        args.get(5).unwrap().unwrap_string(),
        exc_last,
    );
    vec![
        T_HASHMAP::Float(rm.0),
        T_HASHMAP::Float(rm.1),
        T_HASHMAP::Float(rm.2),
        T_HASHMAP::VecF(rm.3),
    ]
}

#[allow(clippy::pedantic)]
pub fn rm_nohesi<T, V>(
    src: &[V],
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

pub fn rm_nohesi_abstr<'a, T>(
    v: &[T],
    _: &Vec<&[T]>,
    args: &ARGS<T>,
    exc_last: &bool
) -> RM_VEC<'a, T>
where  
    T: Float,
{
    vec![T_HASHMAP::Float(rm_nohesi(v, args.first().unwrap().unwrap_f(), exc_last))]
}