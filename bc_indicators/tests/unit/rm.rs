use std::collections::HashMap;

use super::super::common;
use bc_utils::transf;

use bc_indicators::rm::*;


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
        g_rm_sma(vec.iter(), &vec.len(), &common::WINDOW),
        HashMap::from([
            ("src", vec![&3.0, &4.0,]),
        ]),
    );
}

#[test]
fn t_rm_ema_1() {
    let vec = common::g_vec_prices();
    let mut rm = g_rm_ema(vec.iter(), &common::WINDOW);
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
    let vec = common::g_vec_prices();
    let mut rm = g_rm_rma(
        vec.iter(),
        &vec.len(),
        &common::WINDOW
    );
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
    let vec = common::g_vec_prices();
    let mut rm = g_rm_rsi(
        vec.iter(), 
        &vec.len(),
        &common::WINDOW
    );
    rm.1.insert("res", transf::g_round_float(rm.1["res"], 4));
    rm.2.insert("res", transf::g_round_float(rm.2["res"], 4));
    
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

#[test]
fn t_rm_tqo_1() {
    let src = common::g_vec_prices();
    
    assert_eq!(
        g_rm_tqo(
            src.iter(),
            &src.len(),
            &2,
            &3,
            &4,
            &2,
            &10,
            "linear"
        ),
        (
            HashMap::from([
                ("cpc", -0.008199999999999985),
                ("src", 2.2542,),
                ("reversal", -1.0,),
                ("alpha", 0.4,),
                ("trend", -0.00701623862886396),
            ]),
            HashMap::from([
                ("res", 2.2547332488309144,),
                ("alpha", 0.6666666666666666),
            ]),
            HashMap::from([
                ("res", 2.2550673525292657),
                ("alpha", 0.5),
            ]),
            HashMap::from([
                ("src", vec![
                    0.0002729356185600072, 
                    0.0011837613711360249
                ])
            ]),
        )
    );
}