/// functions that return rm values
/// return ready-made values for reuse
/// in rm functions

use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;

use bc_utils::transf;
use bc_utils::create;

use super::indicators::no_oscillators::trend as trend_no_osc;


pub fn rm_trend_ma<T>() -> FxHashMap<&'static str, T> 
where 
    T: Float,
{
    FxHashMap::from_iter([
        ("trend", T::zero()),
        ("l", T::zero()),
    ])
}

pub fn rm_sma<'a, T, V>(
    src: &'a [V],
    window: &usize,
) -> FxHashMap<&'static str, Vec<&'a T>>
where 
    T: Float,
    T: 'a,
    V: Borrow<T>,
{
    let len = src.len();
    let mut src_new = src
        .iter()
        .skip(len - *window - 1)
        .map(Borrow::borrow)
        .collect::<Vec<&T>>();
    transf::roll_slice1(&mut src_new, &1);
    src_new = src_new
        .into_iter()
        .skip(1)
        .collect();
    FxHashMap::from_iter([("src", src_new)])
}

#[allow(clippy::missing_panics_doc)]
pub fn rm_ema<'a, T, V>(
    src: &[V],
    window: &usize,
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
    for (i, el) in src[len - *window * 10..len - 1]
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

#[allow(clippy::missing_panics_doc)]
pub fn rm_rma<'a, T, V>(
    src: &[V],
    window: &usize,
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
    for (i, el) in src[len - *window * 10..len - 1]
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

#[allow(clippy::implicit_hasher)]
pub fn rm_rsi<'a, T, V>(
    src: &[V],
    window: &usize,
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
    let window_need = *window * 10;
    let len_src = src.len();

    for (i, el) in src[len_src - window_need - 1..]
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
        if i == window_need {
            continue;
        }
        src_l = *el.borrow();
    }
    (
        FxHashMap::from_iter([("src", src_l)]),
        rm_rma(u.as_slice(), window),
        rm_rma(d.as_slice(), window)
    )
}

pub fn rm_tqo<'a, T, V>(
    src: &[V],
    window_ema_fast: &usize,
    window_ema_slow: &usize,
    window_trend: &usize,
    window_noise: &usize,
    add_iters: &usize,
    noise_type: &str,
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
    let len_src = src.len() - 1;
    let alpha_trend = trend_no_osc::alpha_ema(&T::from(*window_trend).unwrap());
    let num_need = *window_noise + *window_trend + *add_iters;
    let src = &src[..len_src];
    let src_take = &src[..=(len_src - num_need + 1)];
    let mut rm_ema_fast = rm_ema(src_take, window_ema_fast);
    let mut rm_ema_slow = rm_ema(src_take, window_ema_slow);
    let mut ema_fast;
    let mut ema_slow;
    let mut reversal = T::zero();
    let mut reversal_l = T::nan();
    let mut cpc= T::zero();
    let mut cpc_l = T::nan();
    let mut trend = T::nan();
    let mut trend_l = T::nan();
    let mut diff = Vec::new();
    let mut src_l = src_take[..len_src - num_need]
        .last()
        .unwrap()
        .borrow();

    for (i, el) in src
        .iter()
        .skip(len_src - num_need)
        .enumerate()
    {
        ema_fast = trend_no_osc::ema_rm(el.borrow(), &mut rm_ema_fast);
        ema_slow = trend_no_osc::ema_rm(el.borrow(), &mut rm_ema_slow);
        reversal = create::sign(ema_fast - ema_slow);
        if reversal == reversal_l {
            cpc = cpc_l + *el.borrow() - *src_l;
            trend = trend_l * (T::one() - alpha_trend) + cpc * alpha_trend;
        } else {
            cpc = T::zero();
            trend = T::zero();
        }
        if i > num_need - *window_noise - 1 {
            if noise_type == "linear" {
                diff.push((cpc - trend).abs());
            } else {
                diff.push((cpc - trend).abs().powi(2));
            }
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