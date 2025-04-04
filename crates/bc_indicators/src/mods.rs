use num_traits::Float;
use rustc_hash::FxHashMap;
use bc_utils_lg::enums::indicators::*;
use bc_utils::transf::{avg, avg_with};

use crate::rm::rm_nohesi;


#[allow(clippy::implicit_hasher)]
pub fn nohesi_rm<T>(
    v: &T,
    hesi: &T,
    rm: &mut FxHashMap<&'static str, T>
) -> T
where 
    T: Float,
{
    let v = *v;
    let hesit = v * *hesi;
    let peak_rm = rm["peak"];
    let btm_rm = rm["btm"];
    let peak;
    let btm ;
    
    if v > peak_rm {
        peak = v;
        btm = v - hesit;
    } else if v < btm_rm {
        peak = v + hesit;
        btm = v;
    } else {
        peak = peak_rm;
        btm = btm_rm;
    }
    rm.insert("peak", peak);
    rm.insert("btm", btm);
    if btm < btm_rm || peak > peak_rm {
        rm.insert("res", v);
        return v;
    }
    rm["res"]
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::ptr_arg)]
#[allow(clippy::pedantic)]
pub fn nohesi_rm_abstr<T>(
    v: &T,
    _: &Vec<&T>,
    args: &Vec<T_ARGS<T>>, 
    rm: &mut Vec<T_HASHMAP<T>>
) -> T 
where 
    T: Float,
{
    nohesi_rm(
        v,
        args
            .first()
            .unwrap()
            .unwrap_f(),
        rm
            .first_mut()
            .unwrap()
            .unwrap_f()
    )
}

pub fn nohesi_f<T>(
    v: &[T],
    hesi: &T,
) -> T
where 
    T: Float,
{
    nohesi_rm(v.last().unwrap(), hesi, &mut rm_nohesi(v, hesi, &true))
}

pub fn avg_abstr<T>(
    v: &T,
    add: &Vec<&T>,
    _: &Vec<T_ARGS<T>>, 
) -> T
where 
    T: Float,
    T: std::ops::AddAssign,
{
    avg_with(v, add.as_slice())
}

pub fn avg_rm_abstr<T>(
    v: &T,
    add: &Vec<&T>,
    _: &Vec<T_ARGS<T>>, 
    _: &mut Vec<T_HASHMAP<T>>
) -> T
where 
    T: Float,
    T: std::ops::AddAssign,
{
    avg_with(v, add.as_slice())
}