/// # module for ml engine and analysis
/// 
/// The module provides utilities for data
/// transformation and calculation

use core::f64;


/// replaces nan values ​​with the specified one
/// 
/// # args
/// * `f64` (num): the number
/// * `f64` (exc_value): the replacement value
/// # returns
/// * `f64`: the replacement value
pub fn g_nz(
    num: f64, 
    exc_value: f64,
) -> f64 {
    if num.is_nan() {
        exc_value
    } else {
        num
    }
}

/// returns vector with replacement nan values
/// 
/// # args
/// * `iter_` (Iterator <If32>): iterator where you need to replace the nan values
/// # returns
/// * `Vec<f64>`: vector with replacement nan values
pub fn g_nz_vec<T>(
    iter_: T,
    exc_value: f64,
) -> Vec<f64>
where 
    T: Iterator<Item = f64>,
{
    iter_
        .map(|num| g_nz(num, exc_value))
        .collect()
}

/// returns normalized value
/// 
/// # args
/// * `Iterator<&f64>` (iter_ ): iterator to get normalized value
/// * `&f64` (to_normalize): number to get normalized value
/// * `f64` (min_new): new minimum value
/// * `f64` (max_new): new maximum value
/// # returns
/// * `f64`: normalized value
pub fn g_normalize<'a, T>(
    iter_: T,
    to_normalize: &f64,
    min_new: f64,
    max_new: f64,
) -> f64
where 
    T: Iterator<Item = &'a f64>,
{
    let mut min_historic: &f64 = &f64::NAN;
    let mut max_historic: &f64 = &f64::NAN;

    for num in iter_ {
        min_historic = if num < min_historic {num} else {min_historic};
        max_historic = if num > max_historic {num} else {min_historic};
    };
    (to_normalize - min_historic)
        / (max_historic - min_historic)
        * (max_new - min_new)
        + min_new
}

/// returns the 1-e10 if number is zero, else number
/// 
/// # args
/// * `f64` (num): the number to validate
/// # returns
/// * `f64`: the validated number
pub fn g_dz(
    num: f64,
) -> f64 {
    if num == 0.0 {
        1e-10
    }
    else {
        num
    }
}

/// returns the nan dropped vector
/// 
/// # args
/// * `Vec<f64>` (vec): the vector to nan drop
/// # returns
/// * `Vec<f64>`: the nan dropped vector
pub fn g_vec_drop_nan(
    vec: Vec<f64>,
) -> Vec<f64> {
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
            g_vec_drop_nan(vec![1.0, f64::NAN, 3.0, 5.0]),
            vec![1.0, 3.0, 5.0]
        );
    }
}
