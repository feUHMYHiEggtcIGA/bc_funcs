use std::borrow::Borrow;

use num_traits::Float;


pub fn sign<T, V>(
    num: V,
) -> T
where 
    T: Float,
    V: Borrow<T>,
{
    let num = *num.borrow();

    if num > T::zero() {
        T::one()
    } else if num < T::zero() {
        -T::one()
    } else {
        T::zero()
    }
}