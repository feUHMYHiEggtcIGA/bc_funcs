use std::borrow::Borrow;

use num_traits::{Float, Num, ToPrimitive};


pub fn vec_positive<'a, T, V>(
    slice: &'a[V],
) -> Vec<&'a T>
where 
    T: Float,
    T: 'a,
    V: Borrow<T>,
{
    slice
        .iter()
        .filter(|v| (**v).borrow() > &T::zero())
        .map(|v| v.borrow())
        .collect()
}

pub fn vec_negative<'a, T, V>(
    slice: &'a[V],
) -> Vec<&'a T>
where 
    T: Float,
    T: 'a,
    V: Borrow<T>,
{
    slice
        .iter()
        .filter(|v| (**v).borrow() < &T::zero())
        .map(|v| v.borrow())
        .collect()
}

pub fn lstrip(
    s: &'static str,
    cut_before: char,
) -> &'static str {
    let mut cut_index: usize = 0;

    for c in s
        .chars()
        .enumerate() 
    {
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
        .enumerate() 
    {
        if c.1 == cut_before {
            cut_index = c.0;
            break;
        }
    }
    &s[..s.len() - cut_index]
}

pub fn nz<T, V>(
    num: V,
    exc_value: V,
) -> V
where 
    T: Float,
    V: Borrow<T>,
{
    if num.borrow().is_nan() { exc_value } else { num }
}

pub fn vec_nz<T, V>(
    slice: &[V],
    exc_value: V,
) -> Vec<V>
where 
    T: Float,
    V: Borrow<T>,
    V: Copy,
{
    slice
        .iter()
        .map(|num| nz(*num, exc_value))
        .collect()
}

pub fn normalize<'a, T, V>(
    slice: &[V],
    to_normalize: V,
    min_new: &T,
    max_new: &T,
) -> T
where 
    T: 'a,
    T: Float,
    T: std::ops::Sub<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    V: Borrow<T>,
    V: std::fmt::Debug + std::fmt::Display,
{
    let mut min_historic = T::infinity();
    let mut max_historic = -T::infinity();

    for num in slice {
        min_historic = if num.borrow() < &min_historic {*num.borrow()} else {min_historic};
        max_historic = if num.borrow() > &max_historic {*num.borrow()} else {max_historic};
    };
    (*to_normalize.borrow() - min_historic)
        / (max_historic - min_historic)
        * (*max_new - *min_new)
        + *min_new
}

pub fn dz<T>(
    num: T,
) -> T 
where 
    T: Float,
{
    if num == T::zero() { T::from(1e-10).unwrap() } else { num }
}

pub fn coll_drop_nan<T, V, C>(
    vec: &[V],
) -> C
where 
    T: Float,
    V: Borrow<T>,
    V: Copy,
    C: FromIterator<V>,
{
    vec
        .iter()
        .filter(|x| !(**x).borrow().is_nan())
        .copied()
        .collect()
}

pub fn abs<T, V>(
    num: V,
) -> T 
where
    T: Num,
    T: PartialOrd,
    T: std::ops::Neg<Output = T>,
    T: Copy,
    V: Borrow<T>,
{
    if num.borrow() < &T::zero() { -*num.borrow() } else { *num.borrow() }
}

pub fn avg<T, V>(
    slice_: &[V],
) -> T
where 
    T: Float,
    T: std::ops::AddAssign<T>,
    V: Borrow<T>,
{
    let mut count = 0;
    let mut sum = T::zero();

    for (i, el) in slice_.iter().enumerate() {
        count = i;
        sum += *el.borrow();
    }
    sum / T::from(count + 1).unwrap()
}

pub fn avg_with<T, V>(
    v: V,
    slice_: &[V],
) -> T 
where 
    T: Float,
    T: std::ops::AddAssign<T>,
    V: Borrow<T>,
{
    let mut count = 0;
    let mut sum = T::zero();

    for (i, el) in slice_.iter().enumerate() {
        count = i;
        sum += *el.borrow();
    }
    (sum + *v.borrow()) / T::from(count + 2).unwrap()
}

pub fn roll_slice1<'a, T>(
    v: &'a mut [T],
    shift: &i8,
)
{    
    let shift_usize = (*shift)
        .abs()
        .to_usize()
        .unwrap();
    
    match shift.cmp(&0) {
        std::cmp::Ordering::Greater => v.rotate_right(shift_usize),
        std::cmp::Ordering::Less => v.rotate_left(shift_usize),
        std::cmp::Ordering::Equal => {}
    }
}

pub fn coll1_roll_replace_el<'a, T, V, C>(
    slice: &mut [V],
    shift: &i8,
    to_replace: V,
) -> C
where 
    T: 'a,
    V: Borrow<T>,
    V: Copy,
    C: FromIterator<V>,
{
    let len = slice.len();
    roll_slice1(slice, shift);
    let iter_ = slice.iter();
    let shift_usize = (*shift)
        .abs()
        .to_usize()
        .unwrap();

    match shift.cmp(&0){
        std::cmp::Ordering::Greater => {
            let num_need = shift_usize;
            iter_
                .enumerate()
                .map(|(i, v)| {
                    if i <= num_need {
                        to_replace
                    } else {
                        *v
                    }
                })
                .collect()
        }
        std::cmp::Ordering::Less => {
            let num_need = (len as i8 + shift) as usize;
            iter_
                .enumerate()
                .map(|(i, v)| {
                    if i >= num_need {
                        to_replace
                    } else {
                        *v
                    }
                })
                .collect()
        }
        std::cmp::Ordering::Equal => iter_.copied().collect()
    }
}

pub fn round_float<T, V>(
    num: V,
    precision: &usize,
) -> T
where 
    T: Float,
    V: Borrow<T>,
    V: Copy,
{
    let mult = T::from(10.0.powi(*precision as i32)).unwrap();
    (*num.borrow() * mult).round() / mult
}
