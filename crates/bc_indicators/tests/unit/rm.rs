use rustc_hash::FxHashMap;
use bc_utils_lg::statics::prices::OPEN;
use bc_utils_lg::statics::settings::WINDOW;

use bc_indicators::rm::*;


#[test]
fn rm_trend_ma_1() {
    assert_eq!(
        rm_trend_ma(),
        FxHashMap::from_iter([
            ("trend", 0.0),
            ("l", 0.0),
        ])
    )
}

#[test]
fn rm_sma_1() {
    assert_eq!(
        rm_sma(OPEN.as_slice(), &WINDOW, &true),
        FxHashMap::from_iter([
            ("src", vec![2.2559, 2.2542,]),
        ]),
    );
}

#[test]
fn rm_sma_skip_1() {
    assert_eq!(
        rm_sma(&OPEN[3..], &WINDOW, &true),
        FxHashMap::from_iter([
            ("src", vec![2.2559, 2.2542,]),
        ]),
    );
}

#[test]
fn rm_ema_1() {
    assert_eq!(
        rm_ema(OPEN.as_slice(), &WINDOW, &true),
        FxHashMap::from_iter([
            ("alpha", 0.6666666666666666),
            ("res", 2.2547332546753873),
        ])
    )
}

#[test]
fn rm_ema_skip_1() {
    assert_eq!(
        rm_ema(&OPEN[2..], &WINDOW, &true),
        FxHashMap::from_iter([
            ("alpha", 0.6666666666666666),
            ("res", 2.2547332546753873),
        ])
    )
}

#[test]
fn rm_rma_1() {
    assert_eq!(
        rm_rma(
            OPEN.as_slice(),
            &WINDOW, 
            &true
        ),
        FxHashMap::from_iter([
            ("alpha", 0.5),
            ("res", 2.2550759944915773),
        ])
    );
}

#[test]
fn rm_rma_skip_1() {
    assert_eq!(
        rm_rma(
            &OPEN[2..],
            &WINDOW, 
            &true
        ),
        FxHashMap::from_iter([
            ("alpha", 0.5),
            ("res", 2.2550759944915773),
        ])
    );
}

#[test]
fn rm_rsi_1() {
    assert_eq!(
        rm_rsi(OPEN.as_slice(), &WINDOW, &true),
        (
            FxHashMap::from_iter([
                ("src", 2.2542),
                ]),
            FxHashMap::from_iter([
                ("alpha", 0.5),
                ("res", 0.000273988723754867),
            ]),
            FxHashMap::from_iter([
                ("alpha", 0.5),
                ("res",  0.0011413162231445394),
            ])
        )
    );
}

#[test]
fn rm_rsi_skip_1() {
    assert_eq!(
        rm_rsi(&OPEN[2..], &WINDOW, &true),
        (
            FxHashMap::from_iter([
                ("src", 2.2542),
                ]),
            FxHashMap::from_iter([
                ("alpha", 0.5),
                ("res", 0.000273988723754867),
            ]),
            FxHashMap::from_iter([
                ("alpha", 0.5),
                ("res",  0.0011413162231445394),
            ])
        )
    );
}

#[test]
fn rm_tqo_1() {    
    assert_eq!(
        rm_tqo_b(
            OPEN.as_slice(),
            &2,
            &3,
            &4,
            &2,
            &10,
            "linear", 
            &true
        ),
        (
            FxHashMap::from_iter([
                ("cpc", -0.008199999999999985),
                ("src", 2.2542,),
                ("reversal", -1.0,),
                ("alpha", 0.4,),
                ("trend", -0.00701623862886396),
            ]),
            FxHashMap::from_iter([
                ("res", 2.254733248799198,),
                ("alpha", 0.6666666666666666),
            ]),
            FxHashMap::from_iter([
                ("res", 2.2550673493608473),
                ("alpha", 0.5),
            ]),
            FxHashMap::from_iter([
                ("src", vec![
                    0.0002729356185600072, 
                    0.0011837613711360249
                ])
            ]),
        )
    );
}

#[test]
fn rm_tqo_skip_1() {
    assert_eq!(
        rm_tqo_b(
            &OPEN[2..],
            &2,
            &3,
            &4,
            &2,
            &10,
            "linear", 
            &true
        ),
        (
            FxHashMap::from_iter([
                ("cpc", -0.008199999999999985),
                ("src", 2.2542,),
                ("reversal", -1.0,),
                ("alpha", 0.4,),
                ("trend", -0.00701623862886396),
            ]),
            FxHashMap::from_iter([
                ("res", 2.254733248799198,),
                ("alpha", 0.6666666666666666),
            ]),
            FxHashMap::from_iter([
                ("res", 2.2550673493608473),
                ("alpha", 0.5),
            ]),
            FxHashMap::from_iter([
                ("src", vec![
                    0.0002729356185600072, 
                    0.0011837613711360249
                ])
            ]),
        )
    );
}

#[test]
fn rm_nohesi_1() {
    assert_eq!(
        rm_nohesi(OPEN.as_slice(), &0.0001, &true),
        FxHashMap::from_iter([
            ("res", 2.2542),
            ("peak", 2.25442542),
            ("btm", 2.2542)
        ])
    );
}

#[test]
fn rm_nohesi_skip_1() {
    assert_eq!(
        rm_nohesi(&OPEN[2..], &0.0001, &true),
        FxHashMap::from_iter([
            ("res", 2.2542),
            ("peak", 2.25442542),
            ("btm", 2.2542)
        ])
    );
}