use bc_utils::transf;

use super::super::common;
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
    let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    assert_eq!(
        rm_sma(vec.as_slice(), &common::WINDOW),
        FxHashMap::from_iter([
            ("src", vec![&3.0, &4.0,]),
        ]),
    );
}

#[test]
fn rm_ema_1() {
    let vec = common::PRICES;
    let mut rm = rm_ema(vec.as_slice(), &common::WINDOW);
    rm.insert(
        "res",
            transf::round_float(rm["res"], &4)
    );

    assert_eq!(
        rm,
        FxHashMap::from_iter([
            ("alpha", 0.6666666666666666),
            ("res", 2.2547),
        ])
    )
}

#[test]
fn rm_rma_1() {
    let vec = common::PRICES;
    let mut rm = rm_rma(
        vec.as_slice(),
        &common::WINDOW
    );
    rm.insert(
        "res",
            transf::round_float(rm["res"], &4)
    );
    
    assert_eq!(
        rm,
        FxHashMap::from_iter([
            ("alpha", 0.5),
            ("res", 2.2551),
        ])
    );
}

#[test]
fn rm_rsi_1() {
    let vec = common::PRICES;
    let mut rm = rm_rsi(
        vec.as_slice(), 
        &common::WINDOW
    );
    rm.1.insert("res", transf::round_float(rm.1["res"], &4));
    rm.2.insert("res", transf::round_float(rm.2["res"], &4));
    
    assert_eq!(
        rm,
        (
            FxHashMap::from_iter([
                ("src", 2.2542),
                ]),
            FxHashMap::from_iter([
                ("alpha", 0.5),
                ("res", 0.0003),
            ]),
            FxHashMap::from_iter([
                ("alpha", 0.5),
                ("res", 0.0011),
            ])
        )
    );
}

#[test]
fn rm_tqo_1() {
    let src = common::PRICES;
    
    assert_eq!(
        rm_tqo(
            src.as_slice(),
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
                ("res", 2.2547332488309144,),
                ("alpha", 0.6666666666666666),
            ]),
            FxHashMap::from_iter([
                ("res", 2.2550673525292657),
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