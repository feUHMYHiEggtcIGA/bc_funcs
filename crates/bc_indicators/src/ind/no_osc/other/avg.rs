use num_traits::Float;
use bc_utils_lg::types::structures::{ARGS, SRC_ARG, SRCS_ARG, BF_VEC};
use bc_utils::nums::avg;


pub fn avg_bf_abstr<T>(
    src: &SRC_ARG<T>,
    _: &ARGS<T>, 
    _: &mut BF_VEC<T>,
) -> T
where 
    T: Float,
    T: std::ops::AddAssign,
{
    avg(src)
}

#[allow(clippy::missing_panics_doc)]
pub fn avg_coll<C, T>(
    src: &SRCS_ARG<T>,
) -> C
where 
    T: Float,
    T: std::iter::Sum,
    T: std::ops::AddAssign,
    C: FromIterator<T>,
{
    let min = src.iter().map(|arr| arr.len()).min().unwrap();
    let new_src = src.iter().map(|arr| &arr[arr.len() - min..]).collect::<Vec<&[T]>>();
    new_src
        [0]
        .iter()
        .enumerate()
        .map(|v| avg({
            let mut sk = new_src[1..].iter().map(|v2| v2[v.0]).collect::<Vec<T>>();
            sk.push(*v.1);
            sk
        }.as_slice()))
        .collect()
}

pub fn avg_coll_abstr<C, T>(
    src: &SRCS_ARG<T>,
    _: &ARGS<T>,
) -> C
where 
    T: Float,
    T: std::iter::Sum,
    T: std::ops::AddAssign,
    C: FromIterator<T>,
{
    avg_coll(src)
}

pub fn avg_f_abstr<T>(
    src: &SRCS_ARG<T>, 
    _: &ARGS<T>
) -> T
where 
    T: Float,
    T: std::iter::Sum,
    T: std::ops::AddAssign,
{
    avg(src[0])
}