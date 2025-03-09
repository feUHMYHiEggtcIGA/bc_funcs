use num_traits::Float;

use bc_utils_ml::transf;


pub fn g_percent<T>(
    past: &T, 
    now: &T,
) -> T 
where
    T: Float,
{
    (*now - *past) / *past
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
        positive / transf::g_dz(negative)
    } else {
        positive / negative
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_percent_1() {
        assert_eq!(
            g_percent(&100.0, &105.0),
            0.05
        );
    }
    
    #[test]
    fn t_profit_factor_1() {
        assert_eq!(
            g_profit_factor(vec![1.0, 2.0, -1.0].iter()),
            3.0,
        )
    }
}