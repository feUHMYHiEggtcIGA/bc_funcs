/// # module for ml engine and analysis
/// 
/// The module provides utilities for data
/// transformation and calculation

use core::f32;


/// replaces nan values ​​with the specified one
/// 
/// # args
/// * `f32` (num): the number
/// * `f32` (exc_value): the replacement value
/// # returns
/// * `f32`: the replacement value
pub fn g_nz(
    num: f32, 
    exc_value: f32,
) -> f32 {
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
/// * `Vec<f32>`: vector with replacement nan values
pub fn g_nz_vec<T>(
    iter_: T,
    exc_value: f32,
) -> Vec<f32>
where 
    T: Iterator<Item = f32>,
{
    iter_
        .map(|num| g_nz(num, exc_value))
        .collect()
}

/// returns normalized value
/// 
/// # args
/// * `Iterator<&f32>` (iter_ ): iterator to get normalized value
/// * `&f32` (to_normalize): number to get normalized value
/// * `f32` (min_new): new minimum value
/// * `f32` (max_new): new maximum value
/// # returns
/// * `f32`: normalized value
pub fn g_normalize<'a, T>(
    iter_: T,
    to_normalize: &f32,
    min_new: f32,
    max_new: f32,
) -> f32
where 
    T: Iterator<Item = &'a f32>,
{
    let mut min_historic: &f32 = &f32::NAN;
    let mut max_historic: &f32 = &f32::NAN;

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
/// * `f32` (num): the number to validate
/// # returns
/// * `f32`: the validated number
pub fn g_dz(
    num: f32,
) -> f32 {
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
/// * `Vec<f32>` (vec): the vector to nan drop
/// # returns
/// * `Vec<f32>`: the nan dropped vector
pub fn g_vec_drop_nan(
    vec: Vec<f32>,
) -> Vec<f32> {
    vec.into_iter()
       .filter(|&x|!x.is_nan())
       .collect()
}

fn main() {
    let vec = vec![1.0, f32::NAN, 3.0, 5.0];
    println!("{:?}", g_nz_vec(vec.into_iter(), 0.0));
}
