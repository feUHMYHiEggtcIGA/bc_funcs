use bc_utils::transf::*;


#[test]
fn positive() {
    let vec = vec![1.0, -43.0, 2.0];
    assert_eq!(
        g_vec_positive(vec.iter()),
        vec![&1.0, &2.0],
    )
}

#[test]
fn negative() {
    let vec = vec![1.0, -43.0, 2.0];
    assert_eq!(
        g_vec_negative(vec.iter()),
        vec![&-43.0],
    )
}

#[test]
fn lstrip() {
    let s = "hello world";
    assert_eq!(g_lstrip(s, 'w'), "world");
}

#[test]
fn test_dropnan_1() {
    assert_eq!(
        g_vec_drop_nan(vec![1.0, std::f64::NAN, 3.0, 5.0]),
        vec![1.0, 3.0, 5.0]
    );
}