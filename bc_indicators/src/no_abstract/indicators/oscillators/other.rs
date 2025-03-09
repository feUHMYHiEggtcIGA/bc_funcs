use std::collections::HashMap;

use num_traits::Float;

use crate::no_abstract::indicators::no_oscillators::trend::{self, g_rma_float};
use bc_utils_ml::transf;

pub fn g_rsi<'a, T>(
    rma1: &T,
    rma2: &T,
) -> T
where 
    T: 'a,
    T: Float,
{
    let one_h = T::from(100.0).unwrap();

    one_h - (one_h / (T::one() + transf::g_dz(*rma1 / *rma2)))
}

pub fn g_rsi_rm<T>(
    src: &T,
    rm: &mut HashMap<&'static str, T>,
    rm_rma1: &mut HashMap<&'static str, T>,
    rm_rma2: &mut HashMap<&'static str, T>,
) -> T
where 
    T: Float,
{
    let change = *src - rm["src"];
    let u = src.max(change.clone());
    let d = src.max(-change.clone());

    let rma1 = trend::g_rma_rm(&u, rm_rma1);
    let rma2 = trend::g_rma_rm(&d, rm_rma2);
    g_rsi(&rma1, &rma2)
}

pub fn g_rsi_float<'a, T, I>(
    src: I,
    window: usize,
) -> T
where 
    T: Float,
    T: 'a,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    I: Iterator<Item = &'a T>,
{
    let mut u: Vec<T> = Vec::new();
    let mut d: Vec<T> = Vec::new();
    let mut src_l = T::nan();

    for (i, el) in src.enumerate() {
        if i == 0 {
            src_l = *el;
            continue;
        }
        let change = *el - src_l;
        u.push(change.max(T::zero()));
        d.push((-change).max(T::zero()));
    }
    let rma1 = g_rma_float(u.iter(), &window);
    let rma2 = g_rma_float(d.iter(), &window);
    g_rsi(&rma1, &rma2)
}

#[cfg(test)]
mod tests {
    // use super::*;
}