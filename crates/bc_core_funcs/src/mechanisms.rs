use std::ops::Index;

use reqwest::Error as Error_req;


pub async fn all_or_nothing<T, F, FUT, AR>(
    func: F,
    args: &AR,
) -> T
where 
    AR: Sized,
    FUT: Future<Output = Result<T, Error_req>>,
    F: Fn(&AR) -> FUT,
{
    let mut res = func(args).await;
    while res.is_err() {
        res = func(args).await;
    }
    res.unwrap()
}

pub async fn one_time<'a, T, O, F, FUT, AR>(
    func: F,
    args: &AR,
) -> T
where 
    AR: Sized,
    for<'c> &'c T: IntoIterator<Item = &'c O>,
    T: Index<usize, Output = O>,
    O: PartialEq,
    F: Fn(&AR) -> FUT,
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

pub async fn one_time_hm<'a, H, T, O, F, FUT, AR>(
    func: F,
    args: &AR,
) -> H
where
    AR: std::marker::Sized,
    F: Fn(&AR) -> FUT,
    FUT: Future<Output = H>,
    for<'b> &'b H: IntoIterator<Item = (&'b &'a str, &'b T)>,
    for<'b> &'b T: IntoIterator<Item = &'b O>,
    T: Index<usize, Output = O>,
    O: PartialEq,
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
