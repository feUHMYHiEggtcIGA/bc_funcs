use std::collections::HashMap;

use super::indicators::no_oscillators::trend;

pub fn g_rm_trend_ma() -> HashMap<&'static str, f64> {
    HashMap::from([
        ("trend", 0.0),
        ("l", 0.0),
    ])
}

pub fn g_rm_ema<'a, I>(
    src: I,
    window: &usize,
) -> HashMap<&'static str, f64>
where I: Iterator<Item = &'a f64>
{
    HashMap::from([
        ("alpha", trend::g_alpha_ema(&(*window as f64))),
        ("res", trend::g_ema_float(src, window))
    ])
}

pub fn g_rm_rma<'a, I>(
    src: I,
    window: &usize,
) -> HashMap<&'static str, f64>
where I: Iterator<Item = &'a f64> 
{
    HashMap::from([
        ("res", trend::g_rma_float(src, window)),
        ("alpha", trend::g_alpha_rma(&(*window as f64))),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rm_trend_ma_1() {
        assert_eq!(
            g_rm_trend_ma(),
            HashMap::from([
                ("trend", 0.0),
                ("l", 0.0),
            ])
        )
    }
}