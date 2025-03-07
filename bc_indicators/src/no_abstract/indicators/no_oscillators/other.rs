use bc_utils_ml::transf;

/// Calculates the percentage change between two values.
///
/// # Arguments
/// * `past`: the previous value.
/// * `now`: the current value.
///
/// # Returns
/// * `f64`: the percentage change.
pub fn g_percent(
    past: &f64, 
    now: &f64,
) -> f64 {
    (now - past) / past
}

/// Calculates the profit factor from an iterator of profit/loss quantities.
/// The profit factor is calculated as the ratio of total 
/// positive profit to total negative loss.
///
/// # Arguments
/// * `pnl_qty`: an iterator of profit/loss quantities.
///
/// # Returns
/// * `f64`: the profit factor.
///
/// # Note
/// This function will panic if there are no positive profits 
/// (i.e., `positive` is zero) because it attempts to divide by zero.
pub fn g_profit_factor<'a, I>(
    pnl_qty: I,
) -> f64 
where I: Iterator<Item = &'a f64>
{
    let mut negative = 0.0;
    let mut positive = 0.0;

    for el in pnl_qty {
        if *el < 0.0 {
            negative += el
        } else if *el > 0.0 {
            positive += el
        }
    }
    negative = negative.abs();
    match negative {
        0.0 => positive / transf::g_dz(negative),
        _ => positive / negative,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_1() {
        assert_eq!(
            g_percent(&100.0, &105.0),
            0.05
        );
    }
    
    #[test]
    fn profit_factor_1() {
        assert_eq!(
            g_profit_factor(vec![1.0, 2.0, -1.0].iter()),
            3.0,
        )
    }
    
    #[test]
    fn profit_factor_2() {
        let res = g_profit_factor(vec![1.0, 2.0].iter());

        assert_ne!(
            res,
            3.0
        );
        assert_ne!(
            res,
            std::f64::NAN,
        );
    }
}