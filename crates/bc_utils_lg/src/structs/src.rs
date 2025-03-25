pub struct SrcEl {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

pub struct SrcElT {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub time: f64,
}

pub struct SrcElTV {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub time: f64,
    pub volume: f64,
}

pub struct SrcElTVT {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub time: f64,
    pub volume: f64,
    pub turnover: f64,
}

pub struct Src<'a> {
    pub open: &'a [f64],
    pub high: &'a [f64],
    pub low: &'a [f64],
    pub close: &'a [f64],
}