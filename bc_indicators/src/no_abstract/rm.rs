use std::collections::HashMap;

use num_traits::Float;
use bc_utils::transf;

use super::indicators::no_oscillators::trend;

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
        ("src", src.skip(*len_src - *window).collect::<Vec<&'a T>>())
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
        ("res", trend::g_ema_float(src, window))
    ])
}

pub fn g_rm_rma<'a, I, T>(
    src: I,
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
    let owned_values: Vec<T> = transf::g_vec1_roll(src, 1)
        .into_iter()
        .cloned()
        .collect();

    HashMap::from([
        ("alpha", trend::g_alpha_rma(&T::from(*window).unwrap())),
        ("res", trend::g_rma_float(
            owned_values.iter(),
            window,
        )),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_rm_trend_ma_1() {
        assert_eq!(
            g_rm_trend_ma(),
            HashMap::from([
                ("trend", 0.0),
                ("l", 0.0),
            ])
        )
    }

    #[test]
    fn t_rm_rma_1() {
        assert_ne!(
            g_rm_rma(vec![1.0, 2.0, 3.0, 4.0, 5.0].iter(), &3),
            g_rm_rma(vec![1.0, 2.0, 3.0, 4.0, 6.0].iter(), &3),
        );
    }
}