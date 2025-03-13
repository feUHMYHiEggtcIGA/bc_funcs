use num_traits::Float;


pub fn g_vec_positive<'a, I, T>(
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

pub fn g_vec_negative<'a, I, T>(
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

pub fn g_lstrip(
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

pub fn g_rstrip(
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

pub fn g_abs<T>(
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

pub fn g_avg<'a, T, I>(
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

pub fn g_vec1_roll<'a, T, I>(
    iter_: I,
    shift: i8,
) -> Vec<&'a T>
where 
    I: Iterator<Item = &'a T>,
    T: 'a,
{    
    let shift_usize = shift.abs() as usize;
    let mut res: Vec<&'a T> = iter_.collect();
    
    if shift > 0 {
        res.rotate_right(shift_usize);
    } else if shift < 0 {
        res.rotate_left(shift_usize);
    }
    res
}

// pub fn g_iter_roll<'a, T, I>(
//     iter_: I,
//     shift: i8,
// ) -> Iter<'a, &'a T>
// where 
//     I: Iterator<Item = &'a T>,
//     T: 'a,
// {    
//     let shift_usize = shift.abs() as usize;
//     let mut res: Vec<&'a T> = iter_.collect();
    
//     if shift > 0 {
//         res.rotate_right(shift_usize);
//     } else if shift < 0 {
//         res.rotate_left(shift_usize);
//     }
//     res.iter().ro
// }

pub fn g_vec1_roll_replace_el<'a, T, I>(
    iter_: I,
    iter_len: &usize,
    shift: i8,
    to_replace: &'a T,
) -> Vec<&'a T>
where 
    T: 'a,
    I: Iterator<Item = &'a T>,
{
    let res = g_vec1_roll(iter_, shift);
    let shift_usize = shift.abs() as usize;

    if shift > 0 {
        res
            .iter()
            .enumerate()
            .map(
                |(i, v)| {
                    if i < shift_usize {to_replace} else {*v}
                }
            )
            .collect()
    } else if shift < 0 {
        res
            .iter()
            .enumerate()
            .map(
                |(i, v)| {
                    if i > *iter_len - shift_usize - 1 {to_replace} else {*v}
                }
            )
                .collect()
    } else {
        res
    }
}

pub fn g_round_float<T>(
    num: T,
    precision: usize,
) -> T
where 
    T: Float,
{
    let mult = T::from(10.0.powi(precision as i32)).unwrap();
    (num * mult).round() / mult
}

// fn g_rolling<'a, I, T>(
//     obj: T
// ) -> I
// where 
//     // I: Iterator<Item = &'a T>,
//     // T: 'a,
//     T: Index<T>,
//     T: IndexMut<T>,
//     T: IntoIterator,
//     T: Iterator,
//     T: Sized,
// {
//     obj.map(|v|);
// }