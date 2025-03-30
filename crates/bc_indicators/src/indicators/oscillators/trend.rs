use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;

use bc_utils::create;

use crate::indicators::no_oscillators::trend;
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
/// <https://ru.tradingview.com/v/k3mnaGcQ/>
/// 
/// # Args:
/// * `trend_abs`: ...
/// * `noise`: ...
/// 
/// # Returns
/// * `T`: float number
pub fn tqo_b<T>(
    trend_abs: &T,
    correlation_factor: &T,
    diff_sma: &T,
) -> T 
where 
    T: Float,
{
    *trend_abs / (*trend_abs + *correlation_factor * *diff_sma)
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::implicit_hasher)]
pub fn tqo_b_rm<'a, T>(
    src: &T,
    window_noise: &usize,
    correlation_factor: &T,
    noise_type: &str,
    rm: & mut FxHashMap<&str, T>,
    rm_ema_fast: & mut FxHashMap<&str, T>,
    rm_ema_slow: & mut FxHashMap<&str, T>,
    rm_sma: & mut FxHashMap<&str, Vec<T>>,
) -> T 
where 
    T: Float,
    T: 'a,
    T: std::iter::Sum,
{
    let ema_fast = trend::ema_rm(src, rm_ema_fast);
    let ema_slow = trend::ema_rm(src, rm_ema_slow);

    let reversal = create::sign(ema_fast - ema_slow);
    let cpc = if reversal == rm["reversal"] {rm["cpc"] + *src - rm["src"]} 
        else {T::zero()};
    let trend = if reversal == rm["reversal"] {
            rm["trend"] * (T::one() - rm["alpha"]) + cpc * rm["alpha"]
        } else {T::zero()};
    let diff = (cpc - trend).abs();
    let diff_sma = if noise_type == "linear" {
            trend::sma_rm::<T, T>(
                diff,
                window_noise,
                rm_sma
            )
        } else {
            trend::sma_rm::<T, T>(
                diff.powi(2), 
                window_noise,
                rm_sma
            ).sqrt()
        };
    rm.insert("reversal", reversal);
    rm.insert("cpc", cpc);
    rm.insert("src", *src);
    rm.insert("trend", trend);
    tqo_b(&trend.abs(), correlation_factor, &diff_sma)
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_panics_doc)]
pub fn tqo_b_float<'a, T, V>(
    src: &[V],
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
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    T: std::ops::SubAssign,
    T: std::iter::Sum,
    V: Borrow<T>,
{
    let (
        mut rm, 
        mut rm_ema_fast, 
        mut rm_ema_slow, 
        mut rm_sma,
    ) = rm::rm_tqo(
        src, 
        window_ema_fast, 
        window_ema_slow, 
        window_trend, 
        window_noise, 
        add_iters, 
        noise_type
    );
    tqo_b_rm(
        src.last().unwrap().borrow(), 
        window_noise, 
        correlation_factor, 
        noise_type, 
        &mut rm,
        &mut rm_ema_fast, 
        &mut rm_ema_slow, 
        &mut rm_sma,
    )
}

// wto