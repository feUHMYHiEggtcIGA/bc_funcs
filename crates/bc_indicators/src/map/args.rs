use num_traits::Float;
use bc_utils_lg::enums::indicators::*;
use bc_utils_lg::types::maps::*;
use bc_utils_lg::structs::settings::SETTINGS_IND;


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
#[must_use]
pub fn map_args_ind<T>(
    settings: &'static MAP_LINK<String, SETTINGS_IND>
) -> MAP_ARGS<'static, T>
where  
    T: Float,
{
    settings
        .iter()
        .map(
            |setting| 
            (
                setting.0.as_str(),
                match setting.1.key.as_str() {
                    "sma" | "rma" | "ema" | "rsi" => vec![T_ARGS::<T>::Usize(setting.1.kwargs_usize["window"])],
                    // "tqo_b" => vec![
                    //     T_ARGS::<T>::Usize(setting.1.kwargs_usize["window_ema_fast"]),
                    //     T_ARGS::<T>::Usize(setting.1.kwargs_usize["window_ema_slow"]),
                    //     T_ARGS::<T>::Usize(setting.1.kwargs_usize["window_trend"]),
                    //     T_ARGS::<T>::Usize(setting.1.kwargs_usize["window_noise"]),
                    //     T_ARGS::<T>::Usize(setting.1.kwargs_usize["add_iters"]),
                    //     T_ARGS::<T>::Float(T::from(setting.1.kwargs_f64["correlation_factor"]).unwrap()),
                    //     T_ARGS::<T>::String(setting.1.kwargs_string["noise_type"].clone()),
                    // ],
                    "nohesi" => vec![
                        T_ARGS::<T>::Float(T::from(setting.1.kwargs_f64["hesi"]).unwrap()),
                    ],
                    "avg" | "trend_ma" => vec![],
                    _ => panic!("key indication unknown"),
                }
            )
        )
        .collect()
}