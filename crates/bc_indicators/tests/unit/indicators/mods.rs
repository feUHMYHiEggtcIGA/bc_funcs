use bc_indicators::common;
use bc_indicators::mods::*;
use bc_indicators::rm::*;

#[test]
fn nohesi_rm_1() {
    let mut rm = rm_nohesi(common::OPEN.as_slice(), &0.0001);
    assert_eq!(
        nohesi_rm(common::OPEN.last().unwrap(), &0.0001, &mut rm),
        *common::OPEN.last().unwrap()
    );
}
