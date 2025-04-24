use bc_utils_lg::types::structures_abstr::ARGS;
use reqwest::Error as Error_req;


pub async fn all_or_nothing<'a, T, F, FUT,>(
    func: F,
    args: &'a ARGS<'_, f64>,
) -> T
where 
    FUT: Future<Output = Result<T, Error_req>>,
    F: Fn(&'a ARGS<f64>) -> FUT,
{
    let mut res = func(args).await;
    while res.is_err() {
        res = func(args).await;
    }
    res.unwrap()
}

// pub fn one_time()