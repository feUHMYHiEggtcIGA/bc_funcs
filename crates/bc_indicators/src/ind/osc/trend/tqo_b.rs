/// The Trend-Quality indicator is a trend detection and estimation 
/// tool that is based on a two-step filtering technique. 
/// It measures cumulative price changes over tebf-oriented 
/// semicycles and relates them to “noise”. 
/// The approach reveals congestion and trending periods of the 
/// price movement and focuses on the most important trends, 
/// evaluating their strength in the process. 
/// The indicator is presented banded oscillator fobfat (B-Indicator).
/// 
/// <https://ru.tradingview.com/v/k3mnaGcQ/>

use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::types::structures::*;
use bc_utils::nums::sign;

use crate::ind::no_osc::trend::ema::ema_bf;
use crate::ind::no_osc::trend::sma::sma_bf;
use crate::bf::tqo_b::bf_tqo_b;


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
    args: &ARGS<T>,
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
pub fn tqo_b_bf<'a, T>(
    src: &T,
    window_noise: &usize,
    correlation_factor: &T,
    noise_type: &str,
    bf: & mut FxHashMap<&str, T>,
    bf_ema_fast: & mut FxHashMap<&str, T>,
    bf_ema_slow: & mut FxHashMap<&str, T>,
    bf_sma: & mut FxHashMap<&str, Vec<T>>,
) -> T 
where 
    T: Float,
    T: 'a,
    T: std::iter::Sum,
{
    let ema_fast = ema_bf(src, bf_ema_fast);
    let ema_slow = ema_bf(src, bf_ema_slow);

    let reversal = sign(ema_fast - ema_slow);
    let cpc = if reversal == bf["reversal"] {bf["cpc"] + *src - bf["src"]} 
        else {T::zero()};
    let trend = if reversal == bf["reversal"] {
            bf["trend"] * (T::one() - bf["alpha"]) + cpc * bf["alpha"]
        } else {T::zero()};
    let diff = (cpc - trend).abs();
    let diff_sma = if noise_type == "linear" {
            sma_bf(
                diff,
                window_noise,
                bf_sma
            )
        } else {
            sma_bf(
                diff.powi(2), 
                window_noise,
                bf_sma
            ).sqrt()
        };
    bf.insert("reversal", reversal);
    bf.insert("cpc", cpc);
    bf.insert("src", *src);
    bf.insert("trend", trend);
    tqo_b(&trend.abs(), correlation_factor, &diff_sma)
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_panics_doc)]
pub fn tqo_b_f<'a, T, V>(
    src: &SRC_ARG<V>,
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
        mut bf, 
        mut bf_ema_fast, 
        mut bf_ema_slow, 
        mut bf_sma,
    ) = bf_tqo_b(
        src, 
        window_ema_fast, 
        window_ema_slow, 
        window_trend, 
        window_noise, 
        add_iters, 
        noise_type, 
        &true,
    );
    tqo_b_bf(
        src.last().unwrap().borrow(), 
        window_noise, 
        correlation_factor, 
        noise_type, 
        &mut bf,
        &mut bf_ema_fast, 
        &mut bf_ema_slow, 
        &mut bf_sma,
    )
}

#[allow(clippy::pedantic)]
#[allow(clippy::ptr_arg)]
pub fn tqo_b_bf_abstr< T>(
    src: &SRC_ARG<T>,
    args: &ARGS<T>,
    bf: & mut BF_VEC<T>,
) -> T
where 
    T: Float,
    T: std::iter::Sum,
{
    let (bf1, bf_ema) = bf.split_at_mut(1);
    let (bf_ema_fast, bf_ema_sma) = bf_ema.split_at_mut(1);
    let (bf_ema_slow, bf_sma,) = bf_ema_sma.split_at_mut(1);

    tqo_b_bf(
        src.last().unwrap(),
        args.get(3).expect("arg \"window_noise\" tqo_b_bf not found").unwrap_usize(),
        args.get(1).expect("arg \"correlation_factor\" tqo_b_bf not found").unwrap_f(),
        args.get(2).expect("arg \"noise_type\" tqo_b_bf not found").unwrap_string(),
        bf1.last_mut().expect("bf rsi not found").unwrap_f(),
        bf_ema_fast.last_mut().expect("bf bfa1 in bf rsi not found").unwrap_f(),
        bf_ema_slow.last_mut().expect("bf bfa2 in bf rsi not found").unwrap_f(),
        bf_sma.last_mut().expect("bf sma in tqo_b_bf not found").unwrap_vec_f(),
    )
}

pub fn tqo_b_f_abstr<T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>,
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
        mut bf,
        mut bf_ema_fast,
        mut bf_ema_slow,
        mut bf_sma,
    ) = bf_tqo_b(
        src.last().unwrap(),
        window_ema_fast,
        window_ema_slow,
        window_trend,
        window_noise,
        add_iters,
        noise_type,
        &true,
    );
    tqo_b_bf(
        src.last().unwrap().last().unwrap(), 
        window_noise, 
        correlation_factor, 
        noise_type, 
        &mut bf, 
        &mut bf_ema_fast, 
        &mut bf_ema_slow, 
        &mut bf_sma,
    )
}

pub fn tqo_b_coll<C, T,>(
    src: &SRC_ARG<T>,
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
        mut bf, 
        mut bf_ema_fast, 
        mut bf_ema_slow, 
        mut bf_sma,
    ) = bf_tqo_b(
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
            tqo_b_bf(
                v, 
                window_noise, 
                correlation_factor, 
                noise_type, 
                &mut bf, 
                &mut bf_ema_fast, 
                &mut bf_ema_slow, 
                &mut bf_sma,
            )
        )
        .collect()
}

pub fn tqo_b_coll_abstr<C, T>(
    src: &SRCS_ARG<T>,
    args: &ARGS<T>,
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
        src.last().unwrap(), 
        window_ema_fast, 
        window_ema_slow, 
        window_trend, 
        window_noise, 
        add_iters, 
        correlation_factor, 
        noise_type
    )
}