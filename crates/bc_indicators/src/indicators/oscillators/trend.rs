use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::enums::indicators::*;
use bc_utils_lg::structs::src::*;
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

pub fn window_tqo_b(
    window_ema_fast: &usize,
    window_ema_slow: &usize,
    window_trend: &usize,
    window_noise: &usize,
    add_iters: &usize,
) -> usize {
    *window_noise + (*window_trend).max(*window_ema_fast).max(*window_ema_slow) + *add_iters
}

pub fn window_tqo_b_abstr<T>(
    args: &Vec<T_ARGS<T>>
) -> usize
where 
    T: Float,
{
    window_tqo_b(
        &args[0].unwrap_usize(), 
        &args[1].unwrap_usize(),
        &args[2].unwrap_usize(), 
        &args[3].unwrap_usize(),
        &args[5].unwrap_usize(),
    ) + 1
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
            trend::sma_rm(
                diff,
                window_noise,
                rm_sma
            )
        } else {
            trend::sma_rm(
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
pub fn tqo_b_f<'a, T, V>(
    src: &[V],
    window_ema_fast: &usize,
    window_ema_slow: &usize,
    window_trend: &usize,
    window_noise: &usize,
    add_iters: &usize,
    correlation_factor: &T,
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
    ) = rm::rm_tqo_b(
        src, 
        window_ema_fast, 
        window_ema_slow, 
        window_trend, 
        window_noise, 
        add_iters, 
        noise_type, 
        &true,
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

#[allow(clippy::pedantic)]
#[allow(clippy::ptr_arg)]
pub fn tqo_b_rm_abstr< T>(
    src: &SRC_EL<T>,
    args: &Vec<bc_utils_lg::enums::indicators::T_ARGS<T>>,
    rm: & mut Vec<T_HASHMAP< T>>
) -> T
where 
    T: Float,
    T: std::iter::Sum,
{
    let (rm1, rm_ema) = rm.split_at_mut(1);
    let (rm_ema_fast, rm_ema_sma) = rm_ema.split_at_mut(1);
    let (rm_ema_slow, rm_sma,) = rm_ema_sma.split_at_mut(1);

    tqo_b_rm(
        &src.open,
        args.get(3).expect("arg \"window_noise\" tqo_b_rm not found").unwrap_usize(),
        args.get(1).expect("arg \"correlation_factor\" tqo_b_rm not found").unwrap_f(),
        args.get(2).expect("arg \"noise_type\" tqo_b_rm not found").unwrap_string(),
        rm1.last_mut().expect("rm rsi not found").unwrap_f(),
        rm_ema_fast.last_mut().expect("rm rma1 in rm rsi not found").unwrap_f(),
        rm_ema_slow.last_mut().expect("rm rma2 in rm rsi not found").unwrap_f(),
        rm_sma.last_mut().expect("rm sma in tqo_b_rm not found").unwrap_vec_f(),
    )
}

pub fn tqo_b_f_abstr<T>(
    src: &SRC<T>,
    args: &Vec<T_ARGS<T>>,
) -> T
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    T: std::iter::Sum,
{
    let (
        window_ema_fast,
        window_ema_slow,
        window_trend,
        window_noise,
        add_iters,
        correlation_factor,
        noise_type,
    ) = (
        args.get(0).expect("expectum patronos").unwrap_usize(),
        args.get(1).expect("expectum patronos").unwrap_usize(),
        args.get(2).expect("expectum patronos").unwrap_usize(),
        args.get(3).expect("expectum patronos").unwrap_usize(),
        args.get(4).expect("expectum patronos").unwrap_usize(),
        args.get(5).expect("expectum patronos").unwrap_f(),
        args.get(6).expect("expectum patronos").unwrap_string(),
    );
    let (
        mut rm,
        mut rm_ema_fast,
        mut rm_ema_slow,
        mut rm_sma,
    ) = rm::rm_tqo_b(
        src.open,
        window_ema_fast,
        window_ema_slow,
        window_trend,
        window_noise,
        add_iters,
        noise_type,
        &true,
    );
    tqo_b_rm(
        src.open.last().unwrap(), 
        window_noise, 
        correlation_factor, 
        noise_type, 
        &mut rm, 
        &mut rm_ema_fast, 
        &mut rm_ema_slow, 
        &mut rm_sma,
    )
}

pub fn tqo_b_coll<C, T,>(
    src: &[T],
    window_ema_fast: &usize,
    window_ema_slow: &usize,
    window_trend: &usize,
    window_noise: &usize,
    add_iters: &usize,
    correlation_factor: &T,
    noise_type: &str,
) -> C
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    T: std::iter::Sum,
    C: FromIterator<T>
{
    let w = window_tqo_b(window_ema_fast, window_ema_slow, window_trend, window_noise, add_iters);
    let (
        mut rm, 
        mut rm_ema_fast, 
        mut rm_ema_slow, 
        mut rm_sma,
    ) = rm::rm_tqo_b(
        &src[0..w + 1], 
        window_ema_fast, 
        window_ema_slow, 
        window_trend, 
        window_noise, 
        add_iters, 
        noise_type, 
        &true,
    );
    src
        [w..src.len()]
        .iter()
        .map(
            |v|
            tqo_b_rm(
                v, 
                window_noise, 
                correlation_factor, 
                noise_type, 
                &mut rm, 
                &mut rm_ema_fast, 
                &mut rm_ema_slow, 
                &mut rm_sma,
            )
        )
        .collect()
}

pub fn tqo_b_coll_abstr<C, T>(
    src: &SRC<T>,
    args: &Vec<T_ARGS<T>>,
) -> C
where 
    T: Float,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    T: std::iter::Sum,
    C: FromIterator<T>
{
    let (
        window_ema_fast,
        window_ema_slow,
        window_trend,
        window_noise,
        add_iters,
        correlation_factor,
        noise_type,
    ) = (
        args.get(0).expect("expectum patronos").unwrap_usize(),
        args.get(1).expect("expectum patronos").unwrap_usize(),
        args.get(2).expect("expectum patronos").unwrap_usize(),
        args.get(3).expect("expectum patronos").unwrap_usize(),
        args.get(4).expect("expectum patronos").unwrap_usize(),
        args.get(5).expect("expectum patronos").unwrap_f(),
        args.get(6).expect("expectum patronos").unwrap_string(),
    );
    tqo_b_coll(
        src.open, 
        window_ema_fast, 
        window_ema_slow, 
        window_trend, 
        window_noise, 
        add_iters, 
        correlation_factor, 
        noise_type
    )
}

// wto