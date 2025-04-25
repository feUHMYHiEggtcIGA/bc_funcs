use std::ops::Index;

use bc_utils_lg::structs_and_types::structures_abstr::ARGS;
use reqwest::Error as Error_req;


pub async fn all_or_nothing<'a, T, F, FUT,>(
    func: F,
    args: &'a ARGS<'_, f64, String>,
) -> T
where 
    FUT: Future<Output = Result<T, Error_req>>,
    F: Fn(&'a ARGS<f64, String>) -> FUT,
{
    let mut res = func(args).await;
    while res.is_err() {
        res = func(args).await;
    }
    res.unwrap()
}

pub async fn one_time<'a, T, O, F, FUT,>(
    func: F,
    args: &ARGS<'a, f64, String>,
) -> T
where 
    for<'c> &'c T: IntoIterator<Item = &'c O>,
    T: Index<usize, Output = O>,
    O: PartialEq,
    F: Fn(&ARGS<'a, f64, String>) -> FUT,
    FUT: Future<Output = T>,
{
    let mut res = func(args).await;
    let mut first = &res[0];
    while res
        .into_iter()
        .any(|v| v != first)
    {
        res = func(args).await;
        first = &res[0];
    }
    res
}

pub async fn one_time_hm<'a, H, T, O, F, FUT>(
    func: F,
    args: &'a ARGS<'a, f64, String>,
) -> H
where
    for<'b> &'b H: IntoIterator<Item = (&'b &'a str, &'b T)>,
    for<'b> &'b T: IntoIterator<Item = &'b O>,
    T: Index<usize, Output = O>,
    O: PartialEq,
    F: Fn(&'a ARGS<'a, f64, String>) -> FUT,
    FUT: Future<Output = H>,
{
    let mut res = func(args).await;
    let mut first = &res
        .into_iter()
        .next()
        .unwrap()
        .1
        [0];
    while res
        .into_iter()
        .any(|v| &v.1[0] != first)
    {
        res = func(args).await;
        first = &res
            .into_iter()
            .next()
            .unwrap()
            .1
            [0];
    }
    res
}