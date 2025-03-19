use std::collections::HashMap;

use num_traits::Float;
use bc_utils::transf;
use bc_utils::create;

use super::indicators::no_oscillators::trend as trend_no_osc;


/// functions that return rm values
/// return ready-made values for reuse
/// in rm functions
pub fn g_rm_trend_ma<T>() -> HashMap<&'static str, T> 
where 
    T: Float,
{
    HashMap::from([
        ("trend", T::zero()),
        ("l", T::zero()),
    ])
}

pub fn g_rm_sma<'a, I, T>(
    src: I,
    len_src: &usize,
    window: &usize,
) -> HashMap<&'static str, Vec<&'a T>>
where 
    I: Iterator<Item = &'a T>,
    T: Float,
    T: 'a,
{
    HashMap::from([
        ("src", transf::vec1_roll(
            src, 1)
            .iter()
            .skip(*len_src - *window)
            .map(|v| *v)
            .collect::<Vec<&'a T>>()
        )
    ])
}

pub fn g_rm_ema<'a, I, T>(
    src: I,
    window: &usize,
) -> HashMap<&'static str, T>
where 
    I: Iterator<Item = &'a T>,
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    I: Clone,
{
    // TODO: implement len_src in args
    let len_ = src.clone().count() - 1;

    HashMap::from([
        ("alpha", trend_no_osc::alpha_ema(&T::from(*window).unwrap())),
        ("res", trend_no_osc::ema_float(
            src.take(len_.clone()),
            &len_,
            window
        ))
    ])
}

pub fn g_rm_rma<'a, I, T>(
    src: I,
    len_src: &usize,
    window: &usize,
) -> HashMap<&'static str, T>
where 
    I: Iterator<Item = &'a T>,
    T: Float,
    T: 'a,
    T: Clone,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    HashMap::from([
        ("alpha", trend_no_osc::alpha_rma(&T::from(*window).unwrap())),
        ("res", trend_no_osc::rma_float(
            src.take(*len_src - 1),
            window,
        )),
    ])
}

pub fn g_rm_rsi<'a, T, I>(
    src: I,
    len_src: &usize,
    window: &usize,
) -> (
    HashMap<&'static str, T>,
    HashMap<&'static str, T>, 
    HashMap<&'static str, T>,
)
where 
    T: Float,
    T: 'a,
    I: Iterator<Item = &'a T>,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    I: Clone,
    T: std::fmt::Display,
    T: std::fmt::Debug,
{
    let mut u: Vec<T> = Vec::new();
    let mut d: Vec<T> = Vec::new();
    let mut src_l = T::nan();
    let window_need = *window * 10;

    for (i, el) in src
        .clone()
        .skip(*len_src - window_need.clone())
        .enumerate()
    {
        if i == 0 {
            src_l = *el;
            continue;
        }
        let change = *el - src_l;
        u.push(change.max(T::zero()));
        d.push((-change).max(T::zero()));
        if i == window_need - 1 {
            continue;
        }
        src_l = *el;
    }
    (
        HashMap::from([("src", src_l)]),
        g_rm_rma(u.iter(), &u.len(), &window),
        g_rm_rma(d.iter(), &d.len(), &window)
    )
}

pub fn g_rm_tqo<'a, I, T>(
    src: I,
    len_src: &usize,
    window_ema_fast: &usize,
    window_ema_slow: &usize,
    window_trend: &usize,
    window_noise: &usize,
    add_iters: &usize,
    noise_type: &str,
) -> (
    HashMap<&'static str, T>,
    HashMap<&'static str, T>,
    HashMap<&'static str, T>,
    HashMap<&'static str, Vec<T>>,
)
where 
    T: Float,
    T: 'a,
    I: Iterator<Item = &'a T>,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    I: Clone,
{
    let len_src = *len_src - 1;
    assert!(len_src != 0);

    let src = src.take(len_src);
    let alpha_trend = trend_no_osc::alpha_ema(&T::from(*window_trend).unwrap());
    let num_need = *window_noise + *window_trend + *add_iters;
    let src_take = src.clone().take(len_src - num_need + 1);
    let mut rm_ema_fast = g_rm_ema(src_take.clone(), window_ema_fast);
    let mut rm_ema_slow = g_rm_ema(src_take.clone(), window_ema_slow);
    let mut ema_fast;
    let mut ema_slow;
    let mut reversal = T::zero();
    let mut reversal_l = T::nan();
    let mut cpc= T::zero();
    let mut cpc_l = T::nan();
    let mut trend = T::nan();
    let mut trend_l = T::nan();
    let mut diff = Vec::new();
    let mut src_l = src_take
        .take(len_src - num_need)
        .last()
        .unwrap();

    for (i, el) in src
        .skip(len_src - num_need)
        .enumerate()
    {
        ema_fast = trend_no_osc::ema_rm(el, &mut rm_ema_fast);
        ema_slow = trend_no_osc::ema_rm(el, &mut rm_ema_slow);
        reversal = create::sign(&(ema_fast - ema_slow));
        if reversal == reversal_l {
            cpc = cpc_l + *el - *src_l;
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
        src_l = el;
    }
    (
        HashMap::from([
            ("alpha", alpha_trend),
            ("cpc", cpc),
            ("src", *src_l),
            ("reversal", reversal),
            ("trend", trend),
        ]),
        rm_ema_fast,
        rm_ema_slow,
        HashMap::from([
            ("src", diff)
        ])
    )
}