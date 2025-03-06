use std::collections::HashMap;

use super::indicators::no_oscillators::trend::g_ema_f64_from_iter;

pub fn g_rm_trend_ma() -> HashMap<&'static str, f64> {
    HashMap::from([
        ("trend", 0.0),
        ("l", 0.0),
    ])
}

pub fn g_rm_ema<'a, I>(
    src: I,
    window: usize,
) -> HashMap<&'static str, f64>
where I: Iterator<Item = &'a f64>
{
    HashMap::from([
        ("alpha", 2.0 / (window as f64 + 1.0)),
        ("res", g_ema_f64_from_iter(src, window))
    ])
}