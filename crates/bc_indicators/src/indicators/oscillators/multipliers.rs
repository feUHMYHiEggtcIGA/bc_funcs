use num_traits::Float;


pub fn g_mult_rsi<T>(
    v: &T,
    diff_short: &T,
    diff_long: &T,
    max_v_arg: &T,
) -> T 
where 
    T: Float,
{
    let diff: T;
    let v2: T;
    
    if *v > (*max_v_arg - *diff_short) {
        diff = *diff_short;
        v2 = *max_v_arg - *v;
    } else {
        diff = *diff_long;
        v2 = *v;
    }
    (diff - v2) / diff
}

pub fn g_mult_diff_v<T>(
    v1: &T,
    v2: &T,
    multiplier: &T,
) -> T 
where 
    T: Float,
{
    let min_ = v1.min(*v2);
    let max_ = v1.max(*v2);

    let diff = ((max_ / min_) - T::one()) * *multiplier;
    if diff > T::one() {
        return T::one();
    }
    diff
}
