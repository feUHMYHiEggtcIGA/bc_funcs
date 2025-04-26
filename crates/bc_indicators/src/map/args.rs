use num_traits::Float;
use bc_utils_lg::enums::indicators::*;
use bc_utils_lg::structs_and_types::maps_abstr::*;
use bc_utils_lg::structs_and_types::settings::{SETTINGS_IND, SETTINGS_USED_MODS};


#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
#[must_use]
pub fn map_args_ind<T>(
    settings: &'static Vec<SETTINGS_IND>
) -> MAP_ARGS<T>
where  
    T: Float,
{
    settings
        .iter()
        .map(
            |setting| 
            (
                setting.key_uniq.as_str(),
                match setting.key.as_str() {
                    "sma" | "rma" | "ema" | "rsi" => vec![T_ARGS::<T>::Usize(setting.kwargs_usize["window"])],
                    "tqo_b" => vec![
                        T_ARGS::<T>::Usize(setting.kwargs_usize["window_ema_fast"]),
                        T_ARGS::<T>::Usize(setting.kwargs_usize["window_ema_slow"]),
                        T_ARGS::<T>::Usize(setting.kwargs_usize["window_trend"]),
                        T_ARGS::<T>::Usize(setting.kwargs_usize["window_noise"]),
                        T_ARGS::<T>::Usize(setting.kwargs_usize["add_iters"]),
                        T_ARGS::<T>::Float(T::from(setting.kwargs_f64["correlation_factor"]).unwrap()),
                        T_ARGS::<T>::String(setting.kwargs_string["noise_type"].clone()),
                    ],
                    _ => panic!("key indication unknown"),
                }
            )
        )
        .collect()
}

fn map_args_mod<T>(
    settings: &'static Vec<SETTINGS_USED_MODS>
) -> MAP_ARGS<T>
where 
    T: Float
{
    settings
        .iter()
        .map(
            |mod_| 
            {
                let key = mod_.key.as_str();
                (
                    key,
                    {
                        match key {
                            "avg" => vec![T_ARGS::None(())],
                            "nohesi" => vec![T_ARGS::Float(T::from(mod_.kwargs_f64["hesi"]).expect("invalid cast"))],
                            _ => panic!("mod not found"),
                        }
                    }
                )
            }
        )
        .collect()
}

#[must_use]
pub fn map1_args_mod<T>(
    settings: &'static Vec<SETTINGS_IND>,
) -> MAP1_ARGS<T>
where 
    T: Float,
{
    settings
        .iter()
        .map(
            |setting|
            (
                setting.key_uniq.as_str(),
                map_args_mod(&setting.used_mods)
            )
        )
        .collect()
}

#[must_use]
pub fn map2_args_mods_src<T>(
    settings: &'static Vec<SETTINGS_IND>
) -> MAP2_ARGS<T>
where 
    T: Float,
{
    settings
        .iter()
        .map(
            |setting|
            (
                setting.key_uniq.as_str(),
                setting.used_src
                    .iter()
                    .map(
                        |src|
                        (
                            src.key_uniq.as_str(),
                            map_args_mod(&src.used_mods)
                        )
                    )
                    .collect()
            )
        )
        .collect()
}