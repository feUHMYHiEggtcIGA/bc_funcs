/// all rm functions are dirty because they change rm values.
/// this is done for ease of use, sinceit is not particularly
/// convenient to implement the return of
/// functions, although this is done explicitly.

use std::collections::HashMap;
use std::ops;
use std::borrow::Borrow;

use num_traits::Float;
use bc_utils::transf;


pub fn sma_rm<T, V>(
    src: V,
    window: &usize,
    buff: &mut HashMap<&'static str, Vec<V>>,
) -> T
where
    T: Float,
    T: std::iter::Sum,
    V: Borrow<T>,
    V: Copy,
{
    let buff_res = buff.remove("src").unwrap();
    buff.insert(
        "src", 
        transf::vec1_roll_replace_el(
            buff_res.into_iter(),
            window,
            -1,
            src,
        )
    );
    buff["src"].iter().map(|x| *x.borrow()).sum::<T>() 
    / T::from(*window).unwrap()
}

pub fn ema<T, V>(
    src: V,
    ema_last: V,
    alpha: V,
) -> T 
where 
    T: Float,
    V: Borrow<T>,
    V: Copy,
{
    let alpha = alpha.borrow();
    (*src.borrow() * *alpha) + (*ema_last.borrow() * (T::one() - *alpha))
}

pub fn alpha_ema<T, V>(
    window: V,
) -> T 
where
    T: Float,
    V: Borrow<T>,
    V: Copy,
{
    T::from(2.0).unwrap() / (*window.borrow() + T::one())
}

pub fn ema_rm<T, V>(
    src: V,
    buff: &mut HashMap<&'static str, T>
) -> T 
where 
    T: Float,
    V: Borrow<T>,
    V: Copy,
{
    let res = ema(src.borrow(), &buff["res"], &buff["alpha"]);
    buff.insert("res", res);
    res
}

pub fn ema_float<'a, I, T>(
    iter_: I,
    len_iter: &usize,
    window: &usize,
) -> T
where 
    T: Float,
    T: 'a,
    I: Iterator<Item = &'a T>,
    T: ops::AddAssign,
    T: ops::DivAssign,
    I: Clone,
{
    let mut res = T::zero();
    let window_t = T::from(*window).unwrap();
    
    let alpha = alpha_ema(&window_t);
    for (i, el) in iter_
        .clone()
        .skip(*len_iter - *window * 10)
        .enumerate()
    {
        if i < *window {
            res += *el;
            continue;
        }
        if i == *window {
            res /= window_t;
        }
        res = ema(
            el, 
            &res, 
            &alpha,
        );
    }
    res
}

pub fn rma<T, V>(
    src: V,
    rma_last: V,
    alpha: V,
) -> T 
where 
    T: Float,
    V: Borrow<T>,
    V: Copy,
{
    let alpha = alpha.borrow();
    *alpha * *src.borrow() + (T::one()- *alpha) * *rma_last.borrow()
}

pub fn alpha_rma<T: Float>(
    window: &T,
) -> T {
    T::one() / *window
}

pub fn rma_rm<T: Float>(
    src: &T,
    buff: &mut HashMap<&'static str, T>
) -> T {
    let res = rma(src, &buff["res"], &buff["alpha"]);
    buff.insert("res", res);
    res
}

pub fn rma_float<'a, I, T>(
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
    let window_t = T::from(*window).unwrap();
    
    let alpha = alpha_rma(&window_t);
    for (i, el) in iter_.enumerate() {
        if i < *window {
            res += *el;
            continue;
        }
        if i == *window {
            res /= window_t;
        }
        res = rma(
            el, 
            &res, 
            &alpha,
        );
    }
    res
}

pub fn rational_quadratic_float<'a, I, T>(
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
    transf::nz(current_weight / cumulative_weight, T::zero())
}
