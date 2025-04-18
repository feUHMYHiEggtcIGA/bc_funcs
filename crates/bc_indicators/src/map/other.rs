use bc_utils_lg::structs::settings::SETTINGS_IND;
use num_traits::Float;
use bc_utils_lg::types::funcs_abstr::FUNC_USIZE;
use bc_utils_lg::types::maps_abstr::*;
use bc_utils_lg::types::structures_abstr::ARGS;
use rustc_hash::FxHashMap;

use crate::ind::osc::trend::tqo_b::window_tqo_b_abstr;

pub fn map_func_window_func_bf<T>() -> MAP_FUNC_USIZE<T> 
where 
    T: Float,
{
    MAP_FUNC_USIZE::from_iter([
        ("sma", |a: &ARGS<T>| -> usize {*a.first().unwrap().unwrap_usize() + 1} as FUNC_USIZE<T>),
        ("ema", |a: &ARGS<T>| -> usize {a.first().unwrap().unwrap_usize() * 10 + 1} as FUNC_USIZE<T>),
        ("rma", |a: &ARGS<T>| -> usize {a.first().unwrap().unwrap_usize() * 10 + 1} as FUNC_USIZE<T>),
        ("rsi", |a: &ARGS<T>| -> usize {a.first().unwrap().unwrap_usize() * 10 + 2} as FUNC_USIZE<T>),
        ("tqo_b", window_tqo_b_abstr as FUNC_USIZE<T>),
        ("nohesi", |_: &ARGS<T>| -> usize {3} as FUNC_USIZE<T>),
    ])
}

pub fn map_window_ind<T>(
    map_func_window: &MAP_FUNC_USIZE<T>, 
    settings: &'static Vec<SETTINGS_IND>,
    map_args: &MAP_ARGS<T>
) -> FxHashMap<&'static str, usize>
where 
    T: Float
{
    settings
        .iter()
        .map(
            |setting| {
                let key_uniq = setting.key_uniq.as_str();
                (
                    key_uniq,
                    map_func_window[setting.key.as_str()](
                        &map_args[key_uniq]
                    )
                )
            }
        )
        .collect()
}