use num_traits::Float;


pub fn g_sign<T>(
    num: &T,
) -> T
where 
    T: Float,
{
    if *num > T::zero() {
        T::one()
    } else if *num < T::zero() {
        -T::one()
    } else {
        T::zero()
    }
}