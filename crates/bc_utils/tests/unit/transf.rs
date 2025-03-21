use bc_utils::transf::*;


#[test]
fn positive_1() {
    let vec = vec![1.0, -43.0, 2.0];
    assert_eq!(
        vec_positive(vec.as_slice()),
        vec![&1.0, &2.0],
    )
}

#[test]
fn negative_1() {
    let vec = vec![1.0, -43.0, 2.0];
    assert_eq!(
        vec_negative(vec.as_slice()),
        vec![&-43.0],
    )
}

#[test]
fn lstrip_1() {
    let s = "hello world";
    assert_eq!(lstrip(s, 'w'), "world");
}

#[test]
fn rstrip_1() {
    let s = "hello world";
    assert_eq!(rstrip(s, 'w'), "hello w");
}

#[test]
fn nz_1() {
    assert_eq!(1.0, nz(f64::NAN, 1.0));
}

#[test]
fn nz_2() {
    assert_eq!(&1.0, nz::<f64, &f64>(&f64::NAN, &1.0));
}

#[test]
fn vec_nz_1() {
    assert_eq!(
        vec![1.0, 2.0, 3.0], 
        vec_nz(vec![1.0, 2.0, f64::NAN,].as_slice(), 3.0)
    );
}

#[test]
fn vec_nz_2() {
    assert_eq!(
        vec![&1.0, &2.0, &3.0], 
        vec_nz::<f64, &f64>(vec![&1.0, &2.0, &f64::NAN,].as_slice(), &3.0)
    );
}

#[test]
fn normalize_1() {
    let v = vec![-10.0, -20.0,];
    assert_eq!(normalize(v.as_slice(), -10.0, &0.0, &1.0,), 1.0,);
}

#[test]
fn normalize_2() {
    let v = vec![&-10.0, &-20.0,];
    assert_eq!(normalize(v.as_slice(), &-10.0, &0.0, &1.0,), 1.0,);
}

#[test]
fn dz_1() {
    assert_eq!(
        dz(0.0),
        1e-10,
    );
}

#[test]
fn coll_dropnan_1() {
    let v = vec![1.0, 2.0, f64::NAN];
    assert_eq!(
        coll_drop_nan::<f64, _, Vec<f64>>(v.as_slice()),
        vec![1.0, 2.0]
    );
}

#[test]
fn coll_dropnan_2() {
    let v = vec![&1.0, &2.0, &f64::NAN];
    assert_eq!(
        coll_drop_nan::<f64, _, Vec<&f64>>(v.as_slice()),
        vec![&1.0, &2.0]
    );
}

#[test]
fn abs_1() { assert_eq!(abs(-1), 1); }

#[test]
fn abs_2() { assert_eq!(abs::<i8, _>(&-1), 1); }

#[test]
fn avg_1() { assert_eq!(avg(vec![1.0, 2.0, 3.0].into_iter()), 2.0); }

#[test]
fn roll_slice_1() {
    let mut vec = vec![1, 2, 3];
    roll_slice1(vec.as_mut_slice(), &1);

    assert_eq!(vec, vec![3, 1, 2].as_slice());
}

#[test]
fn roll_slice_2() {
    let mut vec = vec![&1, &2, &3];
    roll_slice1(vec.as_mut_slice(), &1);
    
    assert_eq!(vec, vec![&3, &1, &2].as_slice());
}

#[test]
fn roll_slice_3() {
    let mut vec = vec![&1, &2, &3];
    roll_slice1(vec.as_mut_slice(), &-1);
    
    assert_eq!(vec, vec![&2, &3, &1].as_slice());
}

#[test]
fn vec1_roll_replace_el_1() {
    let mut vec = vec![1, 2, 3];
    let v = coll1_roll_replace_el::<_, _, Vec<i8>>(
        vec.as_mut_slice(),  
        &-1, 
        4,
    );
    
    assert_eq!(v, vec![2, 3, 4],);
}

#[test]
fn vec1_roll_replace_el_2() {
    let mut vec = vec![&1, &2, &3];
    let v = coll1_roll_replace_el::<i8, &i8, Vec<&i8>>(
        vec.as_mut_slice(),  
        &-1, 
        &4,
    );
    
    assert_eq!(v, vec![&2, &3, &4],);
}

#[test]
fn round_float_1() {
    assert_eq!(1.123, round_float(1.123456, &3),)
}
