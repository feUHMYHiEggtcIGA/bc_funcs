use bc_utils_lg::types::structures::SRCS;
use bc_utils_lg::types::maps::MAP;


// pub fn bf_window_src(
//     src: &'static SRCS<f64>,
//     window: &'static usize,
//     exc_last: &bool,
// ) -> MAP<&'static str, Vec<f64>>
// {
//     let sub_ = if *exc_last {1} else {0};
//     MAP::from_iter([
//         ("open", src["open"].clone()[src["open"].len() - window - sub_..src["open"].len() - sub_].to_vec(),),
//         ("high", src["high"].clone()[src["high"].len() - window - sub_..src["open"].len() - sub_].to_vec(),),
//         ("low", src["low"].clone()[src["low"].len() - window - sub_..src["open"].len() - sub_].to_vec(),),
//         ("close", src["close"].clone()[src["close"].len() - window - sub_..src["open"].len() - sub_].to_vec(),),
//     ])
// }