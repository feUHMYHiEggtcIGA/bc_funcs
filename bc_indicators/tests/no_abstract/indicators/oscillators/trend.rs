use std::collections::HashMap;

use num_traits::Float;

use crate::no_abstract::indicators::no_oscillators::trend;
use bc_utils::create;
use bc_utils::transf;


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

pub fn g_tqo_b_rm<'a, T>(
    src: &T,
    window_trend: &T,
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
{
    let ema_fast = trend::g_ema_rm(src, rm_ema_fast);
    let ema_slow = trend::g_ema_rm(src, rm_ema_slow);
    let alpha = trend::g_alpha_ema(window_trend);

    let reversal = create::g_sign(&(ema_fast - ema_slow));
    let cpc = if reversal == rm["reversal"] {rm["cpc"] + *src - rm["src"]} 
        else {T::zero()};
    let trend = if reversal == rm["reversal"] {
            rm["trend"] * (T::one() - alpha) + cpc * alpha
        } else {T::zero()};
    let diff = transf::g_abs(&(cpc - trend));
    let diff_sma;
    if noise_type == "linear" {
        diff_sma = trend::g_sma_rm_nolink(
            diff,
            rm_sma
        );
    } else {
        diff_sma = trend::g_sma_rm_nolink(
            diff.powf(T::from(2).unwrap()), 
            rm_sma
        ).powf(T::from(2).unwrap());
    }
    let trend_abs = trend.abs();
    g_tqo_b(&trend_abs, correlation_factor, &diff_sma)
}

// wto