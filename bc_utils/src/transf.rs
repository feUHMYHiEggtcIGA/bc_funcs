pub fn g_vec_positive<'a, I>(
    iter_: I,
) -> Vec<&'a f64> 
where I: Iterator<Item = &'a f64>,
{
    iter_
        .filter(|v| **v > 0.0)
        .collect()
}

pub fn g_vec_negative<'a, I>(
    iter_: I,
) -> Vec<&'a f64> 
where I: Iterator<Item = &'a f64>,
{
    iter_
        .filter(|v| **v < 0.0)
        .collect()
}

pub fn g_lstrip(
    s: &'static str,
    cut_before: char,
) -> &'static str {
    let mut cut_index: usize = 0;

    for c in s
    .chars()
    .enumerate() {
        if c.1 == cut_before {
            cut_index = c.0;
            break;
        }
    }
    &s[cut_index..]
}

pub fn g_rstrip(
    s: &'static str,
    cut_before: char,
) -> &'static str {
    let mut cut_index: usize = 0;

    for c in s
    .chars()
    .rev()
    .enumerate() {
        if c.1 == cut_before {
            cut_index = c.0;
            break;
        }
    }
    &s[cut_index..]
}