use bc_indicators::common;
use bc_indicators::rm::*;

use rustc_hash::FxHashMap;


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
        rm_sma(common::OPEN.as_slice(), &common::WINDOW),
        FxHashMap::from_iter([
            ("src", vec![&2.2559, &2.2542,]),
        ]),
    );
}

#[test]
fn rm_sma_skip_1() {
    assert_eq!(
        rm_sma(&common::OPEN[3..], &common::WINDOW),
        FxHashMap::from_iter([
            ("src", vec![&2.2559, &2.2542,]),
        ]),
    );
}

#[test]
fn rm_ema_1() {
    assert_eq!(
        rm_ema(common::OPEN.as_slice(), &common::WINDOW),
        FxHashMap::from_iter([
            ("alpha", 0.6666666666666666),
            ("res", 2.254733266399857),
        ])
    )
}

#[test]
fn rm_ema_skip_1() {
    assert_eq!(
        rm_ema(&common::OPEN[2..], &common::WINDOW),
        FxHashMap::from_iter([
            ("alpha", 0.6666666666666666),
            ("res", 2.254733266399857),
        ])
    )
}

#[test]
fn rm_rma_1() {
    assert_eq!(
        rm_rma(
            common::OPEN.as_slice(),
            &common::WINDOW
        ),
        FxHashMap::from_iter([
            ("alpha", 0.5),
            ("res", 2.255084680175781),
        ])
    );
}

#[test]
fn rm_rma_skip_1() {
    assert_eq!(
        rm_rma(
            &common::OPEN[2..],
            &common::WINDOW
        ),
        FxHashMap::from_iter([
            ("alpha", 0.5),
            ("res", 2.255084680175781),
        ])
    );
}

#[test]
fn rm_rsi_1() {
    assert_eq!(
        rm_rsi(common::OPEN.as_slice(), &common::WINDOW),
        (
            FxHashMap::from_iter([
                ("src", 2.2542),
                ]),
            FxHashMap::from_iter([
                ("alpha", 0.5),
                ("res", 0.0002740074157714685),
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
        rm_rsi(&common::OPEN[2..], &common::WINDOW),
        (
            FxHashMap::from_iter([
                ("src", 2.2542),
                ]),
            FxHashMap::from_iter([
                ("alpha", 0.5),
                ("res", 0.0002740074157714685),
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
        rm_tqo(
            common::OPEN.as_slice(),
            &2,
            &3,
            &4,
            &2,
            &10,
            "linear"
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
                ("res", 2.255067349361359),
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
        rm_tqo(
            &common::OPEN[2..],
            &2,
            &3,
            &4,
            &2,
            &10,
            "linear"
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
                ("res", 2.255067349361359),
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
        rm_nohesi(common::OPEN.as_slice(), &0.0001),
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
        rm_nohesi(&common::OPEN[2..], &0.0001),
        FxHashMap::from_iter([
            ("res", 2.2542),
            ("peak", 2.25442542),
            ("btm", 2.2542)
        ])
    );
}