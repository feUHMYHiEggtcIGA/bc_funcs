use std::borrow::Borrow;

use num_traits::Float;

use bc_utils::transf;


pub fn g_percent<T, V>(
    past: V, 
    now: V,
) -> T 
where
    T: Float,
    V: Borrow<T>,
    V: Copy,
{
    let past = past.borrow();
    (*now.borrow() - *past) / *past
}

pub fn g_profit_factor<'a, I, T>(
    pnl_qty: I,
) -> T
where 
    T: Float,
    T: 'a,
    I: Iterator<Item = &'a T>,
    T: std::ops::AddAssign,
{
    let mut negative = T::zero();
    let mut positive = T::zero();
    let zero_ = T::zero();

    for el in pnl_qty {
        if *el < zero_ {
            negative += *el
        } else if *el > zero_ {
            positive += *el
        }
    }
    negative = negative.abs();
    if negative == zero_ {
        positive / transf::dz(negative)
    } else {
        positive / negative
    }
}
