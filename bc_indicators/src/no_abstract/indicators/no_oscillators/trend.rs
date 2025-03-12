/// all rm functions are dirty because they change rm values.
/// this is done for ease of use, sinceit is not particularly
/// convenient to implement the return of
/// functions, although this is done explicitly.

use std::collections::HashMap;
use std::ops;

use num_traits::Float;

use bc_utils::transf;


pub fn g_sma_rm<'a, T>(
    src: &'a T,
    buff_iter: &mut HashMap<&'static str, Vec<&'a T>>,
) -> T 
where
    T: Float,
    T: std::iter::Sum,
    T: 'a,
{
    let len_src = buff_iter["src"].len();

    buff_iter.insert(
        "src", 
        transf::g_vec1_roll_replace_el(
            buff_iter["src"].iter().map(|v| *v),
            &len_src,
            1,
            src,
        )
    );
    buff_iter["src"].iter().map(|x| **x).sum::<T>() / T::from(len_src).unwrap()
}

pub fn g_sma_rm_nolink<'a, T>(
    src: T,
    buff_iter: &mut HashMap<&'static str, Vec<T>>,
) -> T 
where
    T: Float,
    T: std::iter::Sum,
    T: 'a,
{
    let len_src = buff_iter["src"].len();

    buff_iter.insert(
        "src", 
        transf::g_vec1_roll_replace_el(
            buff_iter["src"].iter(),
            &len_src,
            1,
            &src,
        ).iter().map(|v| **v).collect()
    );
    buff_iter["src"].iter().map(|x| *x).sum::<T>() / T::from(len_src).unwrap()
}

pub fn g_ema<T>(
    src: &T,
    ema_last: &T,
    alpha: &T,
) -> T 
where 
    T: Float,
{
    (*src * *alpha) + (*ema_last * (T::one() - *alpha))
}

pub fn g_alpha_ema<T>(
    window: &T,
) -> T 
where
    T: Float,
{
    T::from(2.0).unwrap() / (*window + T::one())
}

pub fn g_ema_rm<T>(
    src: &T,
    buff: &mut HashMap<&'static str, T>
) -> T 
where 
    T: Float
{
    let res = g_ema(src, &buff["res"], &buff["alpha"]);
    buff.insert("res", res);
    res
}

pub fn g_ema_float<'a, I, T>(
    iter_: I,
    window: &usize,
) -> T
where 
    T: Float,
    T: 'a,
    I: Iterator<Item = &'a T>,
    T: ops::AddAssign,
    T: ops::DivAssign,
{
    let mut res = T::zero();
    let window_t = T::from(*window).unwrap();
    
    let alpha = g_alpha_ema(&window_t);
    for (i, el) in iter_.enumerate() {
        if i < *window {
            res += *el;
            continue;
        }
        if i == *window {
            res /= window_t;
        }
        res = g_ema(
            el, 
            &res, 
            &alpha,
        );
    }
    res
}

pub fn g_rma<T: Float>(
    src: &T,
    rma_last: &T,
    alpha: &T,
) -> T {
    *alpha * *src + (T::one()- *alpha) * *rma_last
}

pub fn g_alpha_rma<T: Float>(
    window: &T,
) -> T {
    T::one() / *window
}

pub fn g_rma_rm<T: Float>(
    src: &T,
    buff: &mut HashMap<&'static str, T>
) -> T {
    let res = g_rma(src, &buff["res"], &buff["alpha"]);
    buff.insert("res", res);
    res
}

pub fn g_rma_float<'a, I, T>(
    iter_: I,
    window: &usize,
) -> T
where 
    I: Iterator<Item = &'a T>,
    T: Float,
    T: 'a,
    T: ops::AddAssign,
    T: ops::DivAssign,
{
    let mut res = T::zero();
    let window_t = T::from(*window as f64).unwrap();
    
    let alpha = g_alpha_rma(&window_t);
    for (i, el) in iter_.enumerate() {
        if i < *window {
            res += *el;
            continue;
        }
        if i == *window {
            res /= window_t;
        }
        res = g_rma(
            el, 
            &res, 
            &alpha,
        );
    }
    res
}

pub fn g_rational_quadratic_float<'a, I, T>(
    src: I,
    window: &T,
    relative_weight: &T,
    start_at_bar: &usize,
) -> T 
where 
    I: Iterator<Item = &'a T>,
    T: Float,
    T: 'a,
    T: ops::AddAssign,
    T: ops::MulAssign,
    T: ops::DivAssign,
{
    let mut current_weight: T = T::zero();
    let mut cumulative_weight: T = T::zero();
    let two = T::from(2.0).unwrap();
    let one = T::one();


    for (i, y) in src.enumerate() {
        let w: T = (
            (one + T::from(*start_at_bar - i).unwrap().powf(two)) 
            / ((*window).powf(two) * two * *relative_weight))
                .powf(-(*relative_weight)
        );
        current_weight += w * *y;
        cumulative_weight += w;
    }
    transf::g_nz(current_weight / cumulative_weight, T::zero())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_alpha_ema_1() {
        assert_eq!(g_alpha_ema(&9.0), 0.2);
    }

    #[test]
    fn t_alpha_rma_1() {
        assert_eq!(
            g_alpha_rma(&10.0),
            0.1,
        )
    }
}
