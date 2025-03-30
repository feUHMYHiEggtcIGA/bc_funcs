use num_traits::Float;


#[allow(non_camel_case_types)]
pub struct SRC_EL<T>
where
    T: Float,
{
    pub open: T,
    pub high: T,
    pub low: T,
    pub close: T,
}

#[allow(non_camel_case_types)]
pub struct SRC_EL_T<T>
where
    T: Float,
{
    pub open: T,
    pub high: T,
    pub low: T,
    pub close: T,
    pub time: T,
}

#[allow(non_camel_case_types)]
pub struct SRC_EL_TT<T> 
where
    T: Float,
{
    pub open: T,
    pub high: T,
    pub low: T,
    pub close: T,
    pub time: T,
    pub volume: T,
}

#[allow(non_camel_case_types)]
pub struct SRC_EL_TVT<T>
where
    T: Float,
{
    pub open: T,
    pub high: T,
    pub low: T,
    pub close: T,
    pub time: T,
    pub volume: T,
    pub turnover: T,
}

#[allow(non_camel_case_types)]
pub struct SRC<'a, T>
where
    T: Float,
{
    pub open: &'a [T],
    pub high: &'a [T],
    pub low: &'a [T],
    pub close: &'a [T],
}