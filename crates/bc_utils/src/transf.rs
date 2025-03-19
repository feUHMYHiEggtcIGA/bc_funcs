use std::borrow::Borrow;

use num_traits::Float;


pub fn vec_positive<'a, I, T>(
    iter_: I,
) -> Vec<&'a T> 
where 
    I: Iterator<Item = &'a T>,
    T: Float,
    T: 'a,
{
    iter_
        .filter(|v| **v > T::zero())
        .collect()
}

pub fn vec_negative<'a, I, T>(
    iter_: I,
) -> Vec<&'a T> 
where 
    I: Iterator<Item = &'a T>,
    T: Float,
    T: 'a,
{
    iter_
        .filter(|v| **v < T::zero())
        .collect()
}

pub fn lstrip(
    s: &'static str,
    cut_before: char,
) -> &'static str {
    let mut cut_index: usize = 0;

    for c in s
    .chars()
    .enumerate() {
        if c.1 == cut_before {
            cut_index = c.0;
            break;
        }
    }
    &s[cut_index..]
}

pub fn rstrip(
    s: &'static str,
    cut_before: char,
) -> &'static str {
    let mut cut_index: usize = 0;

    for c in s
    .chars()
    .rev()
    .enumerate() {
        if c.1 == cut_before {
            cut_index = c.0;
            break;
        }
    }
    &s[cut_index..]
}

pub fn nz<T, V>(
    num: V, 
    exc_value: V,
) -> V
where 
    T: Float,
    V: Borrow<T>,
{
    if num.borrow().is_nan() {
        exc_value
    } else {
        num
    }
}

pub fn nz_vec<T, F>(
    iter_: T,
    exc_value: F,
) -> Vec<F>
where 
    T: Iterator<Item = F>,
    F: Float,
{
    iter_
        .map(|num| nz(num, exc_value))
        .collect()
}

pub fn normalize<'a, T, F>(
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

pub fn dz<F: Float>(
    num: F,
) -> F {
    if num == F::zero() {
        F::from(1e-10).unwrap()
    }
    else {
        num
    }
}

pub fn vec_drop_nan<F: Float>(
    vec: Vec<F>,
) -> Vec<F> {
    vec.into_iter()
       .filter(|&x|!x.is_nan())
       .collect()
}

pub fn abs<T>(
    num: &T,
) -> T 
where
    T: Float,
{
    if *num < T::zero() {
        -*num
    } else {
        *num
    }
}

pub fn avg<'a, T, I>(
    iter_: I,
) -> T 
where 
    T: Float,
    T: 'a,
    I: Iterator<Item = &'a T>,
    T: std::ops::AddAssign<T>,
{
    let mut count = 0;
    let mut sum = T::zero();

    for (i, el) in iter_.enumerate() {
        count = i;
        sum += *el;
    }
    sum / T::from(count + 1).unwrap()
}

pub fn vec1_roll<T, I>(
    iter_: I,
    shift: i8,
) -> Vec<T>
where 
    I: Iterator<Item = T>,
{    
    let shift_usize = shift.abs() as usize;
    let mut res: Vec<T> = iter_.collect();
    
    if shift > 0 {
        res.rotate_right(shift_usize);
    } else if shift < 0 {
        res.rotate_left(shift_usize);
    }
    res
}

pub fn vec1_roll_replace_el<'a, T, I>(
    iter_: I,
    len_iter: &usize,
    shift: i8,
    to_replace: T,
) -> Vec<T>
where 
    T: 'a,
    T: Copy,
    I: Iterator<Item = T>,
{
    let res = vec1_roll(iter_, shift);
    let shift_usize = shift.abs() as usize;

    if shift > 0 {
        let num_need = shift_usize;
        res
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                if i <= num_need {
                    to_replace
                } else {
                    v
                }
            })
            .collect()
    } else if shift < 0 {
        let num_need = (*len_iter as i8 + shift) as usize;
        res
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                if i >= num_need {
                    to_replace
                } else {
                    v
                }
            })
            .collect()
    } else {
        res
    }
}

pub fn round_float<T, V>(
    num: V,
    precision: usize,
) -> T
where 
    T: Float,
    V: Borrow<T>,
    V: Copy,
{
    let mult = T::from(10.0.powi(precision as i32)).unwrap();
    (*num.borrow() * mult).round() / mult
}
