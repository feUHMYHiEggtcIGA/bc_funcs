/// # module for ml engine and analysis
/// 
/// The module provides utilities for data
/// transformation and calculation

use num_traits::Float;


pub fn g_nz<T: Float>(
    num: T, 
    exc_value: T,
) -> T {
    if num.is_nan() {
        exc_value
    } else {
        num
    }
}

pub fn g_nz_vec<T, F>(
    iter_: T,
    exc_value: F,
) -> Vec<F>
where 
    T: Iterator<Item = F>,
    F: Float,
{
    iter_
        .map(|num| g_nz(num, exc_value))
        .collect()
}

pub fn g_normalize<'a, T, F>(
    iter_: T,
    to_normalize: &F,
    min_new: F,
    max_new: F,
) -> F
where 
    T: Iterator<Item = &'a F>,
    F: 'a,
    F: Float,
    F: std::ops::Sub<Output = F>,
    F: std::ops::Div<Output = F>,
    F: std::ops::Mul<Output = F>,
    F: std::ops::Add<Output = F>,
{
    let mut min_historic = &F::nan();
    let mut max_historic = &F::nan();

    for num in iter_ {
        min_historic = if num < &min_historic {num} else {min_historic};
        max_historic = if num > &max_historic {num} else {min_historic};
    };
    (*to_normalize - *min_historic)
        / (*max_historic - *min_historic)
        * (max_new - min_new)
        + min_new
}

pub fn g_dz<F: Float>(
    num: F,
) -> F {
    if num == F::zero() {
        F::from(1e-10).unwrap()
    }
    else {
        num
    }
}

pub fn g_vec_drop_nan<F: Float>(
    vec: Vec<F>,
) -> Vec<F> {
    vec.into_iter()
       .filter(|&x|!x.is_nan())
       .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropnan() {
        assert_eq!(
            g_vec_drop_nan(vec![1.0, std::f64::NAN, 3.0, 5.0]),
            vec![1.0, 3.0, 5.0]
        );
    }
}
