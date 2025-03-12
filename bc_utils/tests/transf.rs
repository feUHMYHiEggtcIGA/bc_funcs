use bc_utils::transf::*;


#[test]
fn t_positive_1() {
    let vec = vec![1.0, -43.0, 2.0];
    assert_eq!(
        g_vec_positive(vec.iter()),
        vec![&1.0, &2.0],
    )
}

#[test]
fn t_negative_1() {
    let vec = vec![1.0, -43.0, 2.0];
    assert_eq!(
        g_vec_negative(vec.iter()),
        vec![&-43.0],
    )
}

#[test]
fn t_lstrip() {
    let s = "hello world";
    assert_eq!(g_lstrip(s, 'w'), "world");
}

#[test]
fn t_dropnan_1() {
    assert_eq!(
        g_vec_drop_nan(vec![1.0, std::f64::NAN, 3.0, 5.0]),
        vec![1.0, 3.0, 5.0]
    );
}

#[test]
fn t_vec1_roll_replace_el_1() {
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