use bc_utils::other::*;

#[test]
fn roll_slice_res_1() {
    let mut vec = vec![1, 2, 3];
    roll_slice1(vec.as_mut_slice(), &1);
    assert_eq!(vec, &[3, 1, 2]);
}

#[test]
fn roll_slice_res_2() {
    let mut vec = vec![1, 2, 3];
    roll_slice1(vec.as_mut_slice(), &-1);
    assert_eq!(vec, &[2, 3, 1]);
}

#[test]
fn roll_slice_link_1() {
    let mut vec = vec![&1, &2, &3];
    roll_slice1(vec.as_mut_slice(), &1);
    assert_eq!(vec, &[&3, &1, &2]);
}

#[test]
fn roll_slice_link_2() {
    let mut vec = vec![&1, &2, &3];
    roll_slice1(vec.as_mut_slice(), &-1);   
    assert_eq!(vec, &[&2, &3, &1]);
}

#[test]
fn coll1_roll_replace_el_res_1() {
    let mut vec = vec![1, 2, 3];
    let v = coll1_roll_replace_el::<_, _, Vec<i8>>(
        vec.as_mut_slice(),  
        &-1, 
        4,
    );
    assert_eq!(v, vec![2, 3, 4],);
}

#[test]
fn coll1_roll_replace_el_link_1() {
    let mut vec = vec![&1, &2, &3];
    let v = coll1_roll_replace_el::<i8, &i8, Vec<&i8>>(
        vec.as_mut_slice(),  
        &-1, 
        &4,
    );
    assert_eq!(v, vec![&2, &3, &4],);
}

#[test]
fn lstrip_res_1() {
    let s = "hello world";
    assert_eq!(lstrip(s, 'w'), "world");
}

#[test]
fn rstrip_res_1() {
    let s = "hello world";
    assert_eq!(rstrip(s, 'w'), "hello w");
}