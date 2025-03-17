use std::collections::HashMap;

use num_traits::Float;
use bc_utils::create;

use crate::indicators::no_oscillators::trend;
use crate::indicators::no_oscillators::trend::g_ema_rm;
use crate::rm;


/// The Trend-Quality indicator is a trend detection and estimation 
/// tool that is based on a two-step filtering technique. 
/// It measures cumulative price changes over term-oriented 
/// semicycles and relates them to “noise”. 
/// The approach reveals congestion and trending periods of the 
/// price movement and focuses on the most important trends, 
/// evaluating their strength in the process. 
/// The indicator is presented banded oscillator format (B-Indicator).
/// 
/// https://ru.tradingview.com/v/k3mnaGcQ/
/// 
/// # Args:
/// * `trend_abs`: ...
/// * `noise`: ...
/// 
/// # Returns
/// * `T`: float number
pub fn g_tqo_b<T>(
    trend_abs: &T,
    correlation_factor: &T,
    diff_sma: &T,
) -> T 
where 
    T: Float,
{
    *trend_abs / (*trend_abs + *correlation_factor * *diff_sma)
}

pub fn g_tqo_b_float<'a, T, I>(
    src: I,
    len_src: &usize,
    window_ema_fast: &usize,
    window_ema_slow: &usize,
    window_trend: &usize,
    window_noise: &usize,
    correlation_factor: &T,
    add_iters: &usize,
    noise_type: &str,
) -> T
where 
    T: Float,
    T: 'a,
    I: Iterator<Item = &'a T>,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    T: std::ops::SubAssign,
    I: Clone,
    T: std::fmt::Display + std::fmt::Debug,
{
    let alpha_trend = trend::g_alpha_ema(&T::from(*window_trend).unwrap());
    let num_need = *window_noise + *window_trend + *add_iters;
    let src_take = src.clone().take(*len_src - num_need + 1);
    let mut rm_ema_fast = rm::g_rm_ema(src_take.clone(), window_ema_fast);
    let mut rm_ema_slow = rm::g_rm_ema(src_take.clone(), window_ema_slow);
    let mut ema_fast;
    let mut ema_slow;
    let mut reversal;
    let mut reversal_l = T::nan();
    let mut cpc;
    let mut cpc_l = T::nan();
    let mut trend = T::nan();
    let mut trend_l = T::nan();
    let mut diff = T::zero();
    let mut src_l = src_take
        .take(*len_src - num_need)
        .last()
        .unwrap();

    for (i, el) in src
        .skip(*len_src - num_need)
        .enumerate()
    {
        ema_fast = g_ema_rm(el, &mut rm_ema_fast);
        ema_slow = g_ema_rm(el, &mut rm_ema_slow);
        reversal = create::g_sign(&(ema_fast - ema_slow));
        if reversal == reversal_l {
            cpc = cpc_l + *el - *src_l;
            trend = trend_l * (T::one() - alpha_trend) + cpc * alpha_trend;
        } else {
            cpc = T::zero();
            trend = T::zero();
        }
        if i > num_need - *window_noise - 1 {
            if noise_type == "linear" {
                diff += (cpc - trend).abs();
            } else {
                diff += (cpc - trend).abs().powi(2);
            }
        }
        reversal_l = reversal;
        cpc_l = cpc;
        trend_l = trend;
        src_l = el;
    }
    let diff_sma = if noise_type == "linear" {
        diff / T::from(*window_noise).unwrap()
    } else {
        (diff / T::from(*window_noise).unwrap()).sqrt()
    };
    g_tqo_b(
        &trend.abs(), 
        &correlation_factor,
        &diff_sma,
    )
}

pub fn g_tqo_b_rm<'a, T>(
    src: &T,
    window_noise: &usize,
    correlation_factor: &T,
    noise_type: &str,
    rm: &mut HashMap<&'static str, T>,
    rm_ema_fast: &mut HashMap<&'static str, T>,
    rm_ema_slow: &mut HashMap<&'static str, T>,
    rm_sma: &mut HashMap<&'static str, Vec<T>>,
) -> T 
where 
    T: Float,
    T: 'a,
    T: std::iter::Sum,
    T: std::fmt::Display + std::fmt::Debug,
{
    let ema_fast = trend::g_ema_rm(src, rm_ema_fast);
    let ema_slow = trend::g_ema_rm(src, rm_ema_slow);

    let reversal = create::g_sign(&(ema_fast - ema_slow));
    let cpc = if reversal == rm["reversal"] {rm["cpc"] + *src - rm["src"]} 
        else {T::zero()};
    let trend = if reversal == rm["reversal"] {
            rm["trend"] * (T::one() - rm["alpha"]) + cpc * rm["alpha"]
        } else {T::zero()};
    let diff = (cpc - trend).abs();
    let diff_sma;
    if noise_type == "linear" {
        diff_sma = trend::sma_rm::<T, T>(
            diff,
            window_noise,
            rm_sma
        );
    } else {
        diff_sma = trend::sma_rm::<T, T>(
            diff.powi(2), 
            window_noise,
            rm_sma
        ).sqrt();
    }
    rm.insert("reversal", reversal);
    rm.insert("cpc", cpc);
    rm.insert("src", *src);
    rm.insert("trend", trend);
    g_tqo_b(&trend.abs(), correlation_factor, &diff_sma)
}

// wto