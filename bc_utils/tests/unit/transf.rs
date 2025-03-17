use bc_utils::transf::*;


#[test]
fn positive_1() {
    let vec = vec![1.0, -43.0, 2.0];
    assert_eq!(
        g_vec_positive(vec.iter()),
        vec![&1.0, &2.0],
    )
}

#[test]
fn negative_1() {
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
fn dropnan_1() {
    assert_eq!(
        g_vec_drop_nan(vec![1.0, std::f64::NAN, 3.0, 5.0]),
        vec![1.0, 3.0, 5.0]
    );
}

#[test]
fn vec1_roll_1() {
    let vec = vec![1, 2, 3];
    
    assert_eq!(
        g_vec1_roll(vec.iter(), 1),
        vec![&3, &1, &2]
    );
}

#[test]
fn vec1_roll_2() {
    let vec = vec![1, 2, 3];
    
    assert_eq!(
        g_vec1_roll(vec.into_iter(), 1),
        vec![3, 1, 2]
    );
}

#[test]
fn vec1_roll_replace_el_1() {
    let vec = vec![1, 2, 3];
    
    assert_eq!(
        g_vec1_roll_replace_el(
            vec.iter(), 
            &vec.len(), 
            -1, 
            &4,
        ),
        vec![&2, &3, &4],
    );
}

#[test]
fn vec1_roll_replace_el_2() {
    let vec = vec![1, 2, 3];
    
    assert_eq!(
        g_vec1_roll_replace_el(
            vec.clone().into_iter(), 
            &vec.len(), 
            -1, 
            4,
        ),
        vec![2, 3, 4],
    );
}

#[test]
fn round_float_1() {
    assert_eq!(1.123, g_round_float(1.123456, 3),)
}

#[test]
fn nz_1() {
    assert_eq!(1.0, g_nz(std::f64::NAN, 1.0));
}

#[test]
fn nz_2() {
    assert_eq!(&1.0, g_nz::<f64, &f64>(&std::f64::NAN, &1.0));
}