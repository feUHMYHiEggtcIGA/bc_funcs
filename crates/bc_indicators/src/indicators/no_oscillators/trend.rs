/// all rm functions are dirty because they change rm values.
/// this is done for ease of use, sinceit is not particularly
/// convenient to implement the return of
/// functions, although this is done explicitly.

use std::ops;
use std::borrow::Borrow;

use num_traits::Float;
use rustc_hash::FxHashMap;

use bc_utils::transf;


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::implicit_hasher)]
pub fn sma_rm<T, V>(
    src: V,
    window: &usize,
    buff: &mut FxHashMap<&'static str, Vec<V>>,
) -> T
where
    T: Float,
    T: std::iter::Sum,
    V: Borrow<T>,
    V: Copy,
{
    let v = transf::coll1_roll_replace_el::<_, _, Vec<V>>(
        buff
            .remove("src")
            .unwrap()
            .as_mut_slice(), 
        &-1,
        src,
    );
    buff.insert("src", v);
    buff["src"].iter().map(|x| *x.borrow()).sum::<T>() 
        / T::from(*window).unwrap()
}

pub fn ema<T>(
    src: &T,
    ema_last: &T,
    alpha: &T,
) -> T 
where 
    T: Float,
{
    (*src.borrow() * *alpha) + (*ema_last.borrow() * (T::one() - *alpha))
}

#[allow(clippy::missing_panics_doc)]
pub fn alpha_ema<T>(
    window: &T,
) -> T 
where
    T: Float,
{
    T::from(2.0).unwrap() / (*window + T::one())
}

#[allow(clippy::implicit_hasher)]
pub fn ema_rm<T, V>(
    src: V,
    buff: &mut FxHashMap<&'static str, T>
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

#[allow(clippy::missing_panics_doc)]
pub fn ema_float<'a, T, V>(
    src: &[V],
    window: &usize,
) -> T
where 
    T: Float,
    T: 'a,
    T: ops::AddAssign,
    T: ops::DivAssign,
    V: Borrow<T>,
{
    let len_src = src.len();
    let mut res = T::zero();
    let window_t = T::from(*window).unwrap();
    
    let alpha = alpha_ema(&window_t);
    for (i, el) in src
        .iter()
        .skip(len_src - *window * 10)
        .enumerate()
    {
        if i < *window {
            res += *el.borrow();
            continue;
        }
        if i == *window {
            res /= window_t;
        }
        res = ema(
            el.borrow(), 
            &res, 
            &alpha,
        );
    }
    res
}

pub fn rma<T>(
    src: &T,
    rma_last: &T,
    alpha: &T,
) -> T 
where 
    T: Float,
{
    *alpha * *src + (T::one()- *alpha) * *rma_last
}

pub fn alpha_rma<T: Float>(
    window: &T,
) -> T {
    T::one() / *window
}

#[allow(clippy::implicit_hasher)]
pub fn rma_rm<T: Float>(
    src: &T,
    buff: &mut FxHashMap<&'static str, T>
) -> T {
    let res = rma(src, &buff["res"], &buff["alpha"]);
    buff.insert("res", res);
    res
}

#[allow(clippy::missing_panics_doc)]
pub fn rma_float<'a, T, V>(
    src: &[V],
    window: &usize,
) -> T
where 
    T: Float,
    T: 'a,
    T: ops::AddAssign,
    T: ops::DivAssign,
    V: Borrow<T>,
{
    let mut res = T::zero();
    let window_t = T::from(*window).unwrap();
    
    let alpha = alpha_rma(&window_t);
    for (i, el) in src
        .iter()
        .enumerate() 
    {
        if i < *window {
            res += *el.borrow();
            continue;
        }
        if i == *window {
            res /= window_t;
        }
        res = rma(
            el.borrow(), 
            &res, 
            &alpha,
        );
    }
    res
}

#[allow(clippy::missing_panics_doc)]
pub fn rational_quadratic_float<'a,T>(
    src: &[&'a T],
    window: &usize,
    relative_weight: &T,
    start_at_bar: &usize,
) -> T 
where 
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
    let window_t = T::from(*window).unwrap();


    for (i, y) in src
        .iter()
        .skip(src.len() - *window)
        .enumerate()
    {
        let w: T = (
            (one + T::from(*start_at_bar - i).unwrap().powf(two)) 
            / ((window_t).powf(two) * two * *relative_weight))
                .powf(-(*relative_weight)
        );
        current_weight += w * **y;
        cumulative_weight += w;
    }
    transf::nz(current_weight / cumulative_weight, T::zero())
}
