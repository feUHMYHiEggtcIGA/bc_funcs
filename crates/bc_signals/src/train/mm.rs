use std::cmp::{
    max_by_key, 
    min_by_key,
    Ordering::Equal,
};

use bc_utils_lg::types::structures::SRC;

pub fn mm_coll<C>(
    src: &[SRC<f64>],
    key: &str,
    min_distance: &usize,
    max_distance: &usize,
    tp_th: &f64,
    tp_limit: &f64,
) -> C
where
    C: FromIterator<f64>
{ 
    let lenn = src.len();

    (0..src.len())
        .into_iter()
        .map(
            |i| if i <= lenn - max_distance {
                let enum_ = src[i..i + max_distance].iter().map(|v| v[key]).enumerate();
                let min_ = (
                    enum_.clone().min_by(
                        |v1, v2| 
                        v1
                            .1
                            .partial_cmp(&v2.1)
                            .unwrap_or(Equal)
                    )
                        .unwrap_or_default(), 
                    1.0,
                );
                let max_ = (
                    enum_.max_by(
                        |v1, v2| 
                        v1
                            .1
                            .partial_cmp(&v2.1)
                            .unwrap_or(Equal)
                    )
                        .unwrap_or_default(), 
                    -1.0,
                );
                let diff = (max_.0.1 - min_.0.1) / max_.0.1;
                if diff >= *tp_th  && diff <= *tp_limit

                {
                   if  *min_distance <= (min_.0.0.max(max_.0.0) - max_.0.0.min(min_.0.0)) {

                        let res = min_by_key(max_, min_, |v1| v1.0.0);
                        if res.0.0 == 0 {
                             return res.1;
                        }
                        return 0.0;
                    }
                    return 0.0;
                }
                0.0
            } else {0.0}
        )
        .collect()
}
