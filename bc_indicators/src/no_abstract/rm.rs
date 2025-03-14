use std::collections::HashMap;

use num_traits::Float;
use bc_utils::transf;

use super::indicators::no_oscillators::trend;


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

pub fn g_rm_ema<'a, I>(
    src: I,
    window: &usize,
) -> HashMap<&'static str, f64>
where I: Iterator<Item = &'a f64>
{
    HashMap::from([
        ("alpha", trend::g_alpha_ema(&(*window as f64))),
        ("res", trend::g_ema_float(
            transf::g_vec1_roll(
                src,
                1,
            ).into_iter().skip(1),
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
{
    let mut u: Vec<T> = Vec::new();
    let mut d: Vec<T> = Vec::new();
    let mut src_l = T::nan();

    for (i, el) in src
        .clone()
        .enumerate() 
    {
        if i == 0 {
            src_l = *el;
            continue;
        }
        let change = *el - src_l;
        u.push(change.max(T::zero()));
        d.push((-change).max(T::zero()));
        if i == *len_src - 1 {
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
