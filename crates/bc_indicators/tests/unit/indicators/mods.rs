use crate::common;
use bc_indicators::mods::*;
use bc_indicators::rm::*;

#[test]
fn nohesi_rm_1() {
    let mut rm = rm_nohesi(common::PRICES.as_slice(), &0.0001);
    assert_eq!(
        nohesi_rm(common::PRICES.last().unwrap(), &0.0001, &mut rm),
        *common::PRICES.last().unwrap()
    );
}
