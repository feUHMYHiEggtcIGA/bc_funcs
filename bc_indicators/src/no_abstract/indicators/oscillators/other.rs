use std::collections::HashMap;

use num_traits::Float;

use crate::no_abstract::indicators::no_oscillators::trend;

pub fn g_rsi<'a, T>(
    rma1: &T,
    rma2: &T,
) -> T
where 
    T: 'a,
    T: Float,
{
    let one_h = T::from(100.0).unwrap();

    one_h - (one_h / (T::one() + *rma1 / *rma2))
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
    let u = T::zero().max(change.clone());
    let d = T::zero().max(-change.clone());

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
        src_l = *el;
    }
    let rma1 = trend::g_rma_float(u.iter(), &window);
    let rma2 = trend::g_rma_float(d.iter(), &window);
    g_rsi(&rma1, &rma2)
}

#[cfg(test)]
mod tests {
    use crate::no_abstract::rm::g_rm_rsi;

    use super::*;
    use bc_utils::transf;


    const WINDOW: usize = 2;

    #[test]
    fn g_rsi_rm_1() {
        let vec = vec![
            2.2599,
            2.2654, 2.2742, 2.2736, 2.2706, 2.2736, 
            2.2735, 2.2733, 2.2624, 2.2618, 2.2628, 
            2.2649, 2.2591, 2.2577, 2.2546, 2.2584, 
            2.2555, 2.2553, 2.2559, 2.2542, 2.2547,
        ];
        let (mut rm, mut rm_rma1, mut rm_rma2) = g_rm_rsi(
            vec.iter(), 
            &vec.len(), 
            &WINDOW,
        );

        assert_eq!(
            transf::g_round_float(
                g_rsi_rm(
                    &2.2547, 
                    &mut rm, 
                    &mut rm_rma1, 
                    &mut rm_rma2,
                ), 
            4),
            40.41,
        )
    }
}