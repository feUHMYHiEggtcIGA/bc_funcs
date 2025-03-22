use num_traits::Float;
use rustc_hash::FxHashMap;


#[allow(clippy::implicit_hasher)]
pub fn nohesi_rm<T>(
    v: &T,
    hesi: &T,
    rm: &mut FxHashMap<&'static str, T>
) -> T
where 
    T: Float,
{
    let hesit = *v * *hesi;
    let peak_rm = rm["peak"];
    let btm_rm = rm["btm"];
    let peak;
    let btm ;
    
    if *v > peak_rm {
        peak = *v;
        btm = *v - hesit;
    } else if *v < btm_rm {
        peak = *v + hesit;
        btm = *v;
    } else {
        peak = peak_rm;
        btm = btm_rm;
    }
    rm.insert("peak", peak);
    rm.insert("btm", btm);
    if btm < btm_rm || peak > peak_rm {
        rm.insert("res", *v);
        return *v;
    }
    rm["res"]
}