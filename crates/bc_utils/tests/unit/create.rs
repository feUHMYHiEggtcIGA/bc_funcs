use bc_utils::create::*;

#[test]
fn sign_1() {
    assert_eq!(1.0, sign(5.0),);
}

#[test]
fn sign_2() {
    assert_eq!(-1.0, sign(-5.0),);
}

#[test]
fn sign_3() {
    assert_eq!(0.0, sign(0.0),);
}
