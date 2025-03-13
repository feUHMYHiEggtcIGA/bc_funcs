use std::collections::HashMap;

use num_traits::Float;
use bc_utils::transf;

use super::indicators::no_oscillators::trend;

pub fn g_rm_trend_ma<T>() -> HashMap<&'static str, T> 
where 
    T: Float,
{
    HashMap::from([
        ("trend", T::zero()),
        ("l", T::zero()),
    ])
}

pub fn g_rm_sma<'a, I, T>(
    src: I,
    len_src: &usize,
    window: &usize,
) -> HashMap<&'static str, Vec<&'a T>>
where 
    I: Iterator<Item = &'a T>,
    T: Float,
    T: 'a,
{
    HashMap::from([
        ("src", transf::g_vec1_roll(
            src, 1)
            .iter()
            .skip(*len_src - *window)
            .map(|v| *v)
            .collect::<Vec<&'a T>>()
        )
    ])
}

pub fn g_rm_ema<'a, I>(
    src: I,
    window: &usize,
) -> HashMap<&'static str, f64>
where I: Iterator<Item = &'a f64>
{
    HashMap::from([
        ("alpha", trend::g_alpha_ema(&(*window as f64))),
        ("res", trend::g_ema_float(
            transf::g_vec1_roll(
                src,
                1,
            ).into_iter().skip(1),
            window
        ))
    ])
}

pub fn g_rm_rma<'a, I, T>(
    src: I,
    window: &usize,
) -> HashMap<&'static str, T>
where 
    I: Iterator<Item = &'a T>,
    T: Float,
    T: 'a,
    T: Clone,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
{
    let owned_values: Vec<T> = transf::g_vec1_roll(src, 1)
        .into_iter()
        .cloned()
        .collect();

    HashMap::from([
        ("alpha", trend::g_alpha_rma(&T::from(*window).unwrap())),
        ("res", trend::g_rma_float(
            owned_values.iter(),
            window,
        )),
    ])
}

pub fn g_rm_rsi<'a, T, I>(
    src: I,
    len_src: &usize,
    window: &usize,
) -> (
    HashMap<&'static str, T>,
    HashMap<&'static str, T>, 
    HashMap<&'static str, T>,
)
where 
    T: Float,
    T: 'a,
    I: Iterator<Item = &'a T>,
    T: std::ops::AddAssign,
    T: std::ops::DivAssign,
    I: Clone,
{
    let mut u: Vec<T> = Vec::new();
    let mut d: Vec<T> = Vec::new();
    let mut src_l = T::nan();

    for (i, el) in src.clone().enumerate() {
        if i == 0 {
            src_l = *el;
            continue;
        }
        let change = *el - src_l;
        u.push(change.max(T::zero()));
        d.push((-change).max(T::zero()));
        if i == *len_src - 1 {
            continue;
        }
        src_l = *el;
    }

    (
        HashMap::from([("src", *src.skip(*len_src - 2).next().unwrap())]),
        g_rm_rma(u.iter(), &window),
        g_rm_rma(d.iter(), &window)
    )
}

#[cfg(test)]
mod tests {
    use bc_utils::transf::g_round_float;

    use super::*;


    const WINDOW: usize = 2;

    #[test]
    fn t_rm_trend_ma_1() {
        assert_eq!(
            g_rm_trend_ma(),
            HashMap::from([
                ("trend", 0.0),
                ("l", 0.0),
            ])
        )
    }

    #[test]
    fn t_rm_sma_1() {
        let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        assert_eq!(
            g_rm_sma(vec.iter(), &vec.len(), &WINDOW),
            HashMap::from([
                ("src", vec![&3.0, &4.0,]),
            ]),
        );
    }
    
    #[test]
    fn t_rm_ema_1() {
        let vec = vec![
            2.2599,
            2.2654, 2.2742, 2.2736, 2.2706, 2.2736, 
            2.2735, 2.2733, 2.2624, 2.2618, 2.2628, 
            2.2649, 2.2591, 2.2577, 2.2546, 2.2584, 
            2.2555, 2.2553, 2.2559, 2.2542, 2.2547,
        ];
        let mut rm = g_rm_ema(vec.iter(), &WINDOW);
        rm.insert(
            "res",
             transf::g_round_float(rm["res"], 4)
        );

        assert_eq!(
            rm,
            HashMap::from([
                ("alpha", 0.6666666666666666),
                ("res", 2.2547),
            ])
        )
    }

    #[test]
    fn t_rm_rma_1() {
        let vec = vec![
            2.2599,
            2.2654, 2.2742, 2.2736, 2.2706, 2.2736, 
            2.2735, 2.2733, 2.2624, 2.2618, 2.2628, 
            2.2649, 2.2591, 2.2577, 2.2546, 2.2584, 
            2.2555, 2.2553, 2.2559, 2.2542, 2.2547,
        ];
        let mut rm = g_rm_rma(vec.iter(), &WINDOW);
        rm.insert(
            "res",
             transf::g_round_float(rm["res"], 4)
        );
        
        assert_eq!(
            rm,
            HashMap::from([
                ("alpha", 0.5),
                ("res", 2.2551),
            ])
        );
    }

    #[test]
    fn t_rm_rsi_1() {
        let vec = vec![
            2.2599,
            2.2654, 2.2742, 2.2736, 2.2706, 2.2736, 
            2.2735, 2.2733, 2.2624, 2.2618, 2.2628, 
            2.2649, 2.2591, 2.2577, 2.2546, 2.2584, 
            2.2555, 2.2553, 2.2559, 2.2542, 2.2547,
        ];
        let mut rm = g_rm_rsi(
            vec.iter(), 
            &vec.len(),
            &WINDOW
        );
        rm.1.insert("res", g_round_float(rm.1["res"], 4));
        rm.2.insert("res", g_round_float(rm.2["res"], 4));
        
        assert_eq!(
            rm,
            (
                HashMap::from([
                    ("src", 2.2542),
                    ]),
                HashMap::from([
                    ("alpha", 0.5),
                    ("res", 0.0003),
                ]),
                HashMap::from([
                    ("alpha", 0.5),
                    ("res", 0.0011),
                ])
            )
        );
    }
}