use std::collections::HashMap;

use num_traits::Float;
use bc_utils::transf;
use bc_utils::create;

use super::indicators::no_oscillators::trend::g_ema_rm;
use super::indicators::no_oscillators::trend::{self, g_ema_float};


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
        ("src", transf::g_vec1_roll(
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
        ("alpha", trend::g_alpha_ema(&T::from(*window).unwrap())),
        ("res", trend::g_ema_float(
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
        ("alpha", trend::g_alpha_rma(&T::from(*window).unwrap())),
        ("res", trend::g_rma_float(
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

// pub fn g_rm_tqo_b<'a, T, I>(
//     src: I,
//     len_src: &usize,
//     window_ema_fast: &usize,
//     window_ema_slow: &usize,
//     window_noise: &usize,
//     window_trend: &usize,
// ) -> (
//     HashMap<&'static str, T>,
//     HashMap<&'static str, T>,
//     HashMap<&'static str, T>,
//     HashMap<&'static str, Vec<T>>,
// )
// where
//     I: Iterator<Item = &'a T>,
//     T: Float,
//     T: 'a,
//     T: std::ops::AddAssign,
//     T: std::ops::DivAssign,
//     I: Clone,
// {
//     let src_m1 = src.take(*len_src - 1);
//     let src_el_l = src_m1.last().unwrap();
//     let alpha = trend::g_alpha_ema(&T::from(*window_trend).unwrap());
    
//     let rm_ema_fast = g_rm_ema(src_m1.clone(), window_ema_fast);
//     let rm_ema_slow = g_rm_ema(src_m1.clone(), window_ema_slow);
//     let reversal_l = create::g_sign(&(rm_ema_fast["res"] - rm_ema_slow["res"]));
//     let ema_fast = g_ema_rm(src_el_l, &mut rm_ema_fast);
//     let ema_slow = g_ema_rm(src_el_l, &mut rm_ema_slow);
//     let reversal = create::g_sign(&(ema_fast - ema_slow));
//     let cpc = if reversal == reversal_l {
//         T::zero() + *src_el_l - *src_m1.take(*len_src - 1).last().unwrap()
//     } else {T::zero()};
//     (
//         HashMap::from([
//             ("alpha", alpha),
//             ("reversal", reversal),
//             ("cpc", cpc),
//             (
//                 "trend", if reversal == reversal_l {
//                     T::zero() * (T::one() - alpha) + cpc * alpha
//                 } else {T::zero()}
//             )
//         ]),
//     )
// }