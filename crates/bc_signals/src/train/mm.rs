use std::cmp::{
    max_by_key, 
    min_by_key, 
    Ordering,
};

use bc_utils_lg::types::structures::SRC;

pub fn mm_coll(
    src: &[SRC<f64>],
    window: &usize,
    min_distance: &usize,
    tp_th: &f64,
    tp_limit: &f64,
) -> (Vec<f64>, Vec<f64>)
{
    let src_len = src.len();
    let mut entrances = vec![0.0; src_len];
    let mut exits = vec![0.0; src_len];
    for (i, el) in src
        .windows(*window)
        .enumerate()
    {
        let (min, max) = el
            .iter()
            .enumerate()
            .fold(
                ((0, f64::INFINITY), (0, f64::NEG_INFINITY),),
                |init, v| (
                    match init.0.1.partial_cmp(&v.1["low"]) {
                        Some(Ordering::Greater) => (v.0 + i, v.1["low"]),
                        _ => init.0,
                    },
                    match init.1.1.partial_cmp(&v.1["high"]) {
                        Some(Ordering::Less) => (v.0 + i, v.1["high"]),
                        _ => init.1,
                    }
                )
            );
        let index_min = min_by_key(&min, &max, |v| v.0);
        let index_max = max_by_key(&min, &max, |v| v.0);
        let range_ = (index_max.1 / index_min.1 - 1.0).abs();
        if &(index_max.0 - index_min.0) >= min_distance 
            && &range_ >= tp_th
            && &range_ <= tp_limit
        {
            if index_min.1 < index_max.1 {
                entrances[index_min.0] = -1.0;
                exits[index_max.0] = 1.0;
            } else {
                entrances[index_min.0] = 1.0;
                exits[index_max.0] = -1.0;
            }
        }
    }
    (entrances, exits)    
}