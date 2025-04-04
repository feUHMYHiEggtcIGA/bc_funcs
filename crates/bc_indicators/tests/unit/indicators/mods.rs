use bc_utils_lg::statics::prices::OPEN;
use bc_indicators::mods::*;
use bc_indicators::rm::*;

#[test]
fn nohesi_rm_1() {
    let mut rm = rm_nohesi(OPEN.as_slice(), &0.0001, &true,);
    assert_eq!(
        nohesi_rm(OPEN.last().unwrap(), &0.0001, &mut rm),
        *OPEN.last().unwrap()
    );
}
