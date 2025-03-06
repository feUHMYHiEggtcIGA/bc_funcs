/// all rm functions are dirty because they change rm values.
/// this is done for ease of use, sinceit is not particularly
/// convenient to implement the return of
/// functions, although this is done explicitly.

use std::collections::HashMap;


/// Calculates the Exponential Moving Average (EMA) for a given value and previous EMA.
///
/// # Arguments
/// * `src`: the current value.
/// * `ema_last`: the previous EMA value.
/// * `alpha`: the smoothing coefficient.
///
/// # Returns
/// * `f64`: the current EMA value.
pub fn g_ema(
    src: &f64,
    ema_last: &f64,
    alpha: &f64,
) -> f64 {
    (src * alpha) + (ema_last * (1.0 - alpha))
}

/// Calculates the alpha value for Exponential Moving Average (EMA) based on the given window size.
///
/// # Arguments
/// * `window`: the window size.
///
/// # Returns
/// * `f64`: the alpha value for EMA.
pub fn g_alpha_ema(
    window: &f64,
) -> f64 {
    2.0 / (*window + 1.0)
}

/// Calculates EMA with the last result stored in a buffer.
///
/// # Arguments
/// * `src`: the current value.
/// * `buff`: a buffer to store the last result and smoothing coefficient.
///
/// # Returns
/// * `f64`: the current EMA value.
pub fn g_ema_rm(
    src: &f64,
    buff: &mut HashMap<&'static str, f64>
) -> f64 {
    let res = g_ema(src, &buff["res"], &buff["alpha"]);
    buff.insert("res", res);
    res
}

/// Calculates EMA for an iterator of values with a specified window.
///
/// # Arguments
/// * `iter_`: an iterator of values.
/// * `window`: the size of the window for averaging.
///
/// # Returns
/// * `f64`: the last EMA value.
pub fn g_ema_f64_from_iter<'a, I>(
    iter_: I,
    window: &usize,
) -> f64
where I: Iterator<Item = &'a f64>
{
    let mut res = 0.0;
    let alpha = 2.0 / (*window as f64 + 1.0);
    
    for (i, el) in iter_.enumerate() {
        if i < *window {
            res += el;
            continue;
        }
        if i == *window {
            res /= *window as f64;
        }
        res = g_ema(
            el, 
            &res, 
            &alpha,
        );
    }
    res
}

/// Calculates EMA for an iterator of values with a specified window and returns a vector of values.
///
/// # Arguments
/// * `iter_`: an iterator of values.
/// * `window`: the size of the window for averaging.
///
/// # Returns
/// * `Vec<f64>`: a vector of EMA values.
pub fn g_ema_vec_from_iter<'a, I>(
    iter_: I,
    window: &usize,
) -> Vec<f64>
where I: Iterator<Item = &'a f64>
{
    let mut res: Vec<f64> = (0..*window)
        .map(|_| core::f64::NAN)
        .collect();
    let mut res_last = 0.0;
    let alpha = 2.0 / (*window as f64 + 1.0);
    
    for (i, el) in iter_.enumerate() {
        if i < *window {
            res_last += el;
            continue;
        }
        if i == *window {
            res_last /= *window as f64;
            res.push(res_last); // Corrected for proper indexing
        }
        res.push(g_ema(
            el, 
            &res_last, 
            &alpha,
        ));
    }
    res
}

/// Calculates the Rolling Moving Average (RMA) for a given value and previous RMA.
///
/// # Arguments
/// * `src`: the current value.
/// * `rma_last`: the previous RMA value.
/// * `alpha`: the smoothing coefficient.
///
/// # Returns
/// * `f64`: the current RMA value.
pub fn g_rma(
    src: &f64,
    rma_last: &f64,
    alpha: &f64,
) -> f64 {
    *alpha * *src + (1.0 - *alpha) * *rma_last
}

/// Calculates the alpha value for Risk Management Average (RMA) based on the given window size.
///
/// # Arguments
/// * `window`: the window size.
///
/// # Returns
/// * `f64`: the alpha value for RMA.
pub fn g_alpha_rma(
    window: &f64,
) -> f64 {
    1.0 / *window
}

/// Calculates RMA with the last result stored in a buffer.
///
/// # Arguments
/// * `src`: the current value.
/// * `buff`: a buffer to store the last result and smoothing coefficient.
///
/// # Returns
/// * `f64`: the current RMA value.
pub fn g_rma_rm(
    src: &f64,
    buff: &mut HashMap<&'static str, f64>
) -> f64 {
    let res = g_rma(src, &buff["res"], &buff["alpha"]);
    buff.insert("res", res);
    res
}

/// Calculates RMA for an iterator of values with a specified window.
///
/// # Arguments
/// * `iter_`: an iterator of values.
/// * `window`: the size of the window for averaging.
///
/// # Returns
/// * `f64`: the last RMA value.
pub fn g_rma_f64_from_iter<'a, I>(
    iter_: I,
    window: &usize,
) -> f64 
where I: Iterator<Item = &'a f64>
{
    let mut res = 0.0;
    let alpha = 1.0 / *window as f64;
    
    for (i, el) in iter_.enumerate() {
        if i < *window {
            res += el;
            continue;
        }
        if i == *window {
            res /= *window as f64;
        }
        res = g_rma(
            el, 
            &res, 
            &alpha,
        );
    }
    res
}

/// Calculates RMA for an iterator of values with a specified window and returns a vector of values.
///
/// # Arguments
/// * `iter_`: an iterator of values.
/// * `window`: the size of the window for averaging.
///
/// # Returns
/// * `Vec<f64>`: a vector of RMA values.
pub fn g_rma_vec_from_iter<'a, I>(
    iter_: I,
    window: &usize,
) -> Vec<f64>
where I: Iterator<Item = &'a f64>
{
    let mut res: Vec<f64> = (0..*window).map(|_| std::f64::NAN)
        .collect();
    let mut res_last: f64 = 0.0;
    let alpha = 1.0 / *window as f64;
    
    for (i, el) in iter_.enumerate() {
        if i < *window {
            res_last += el;
            continue;
        }
        if i == *window {
            res_last /= *window as f64;
            res.push(res_last);
        }
        res.push(g_rma(
            &el, 
            &res_last, 
            &alpha,
        ));
    }
    res
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        assert_eq!(2 + 2, 4);
    }
}
