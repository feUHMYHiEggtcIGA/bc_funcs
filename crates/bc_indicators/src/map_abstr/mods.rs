use std::borrow::Borrow;
use std::ops::AddAssign;
use std::iter::Sum;

use bc_utils_lg::enums::indicators::T_ARGS;
use bc_utils_lg::types::structures_abstr::ARGS;
use bc_utils_lg::types::structures_abstr::BF_VEC;
use num_traits::Float;
use bc_utils_lg::types::funcs_abstr::*;
use bc_utils_lg::types::maps_abstr::*;

use crate::mods::{
    avg::{avg_abstr, avg_coll_abstr, avg_from_coll_abstr, avg_bf_abstr},
    nohesi::{nohesi_bf_abstr, nohesi_coll_abstr, nohesi_f_abstr},
};


pub fn map_mod_bf<T>() -> MAP_MOD_T_BF<T>
where 
    T: Float,
    T: AddAssign,
{
    // let va = nohesi_bf_abstr::<f64, &f64>(&2.0, &[&1.0], &vec![T_ARGS::Float(1.0)], &mut BF_VEC::default());
    // let res = MAP_MOD_T_BF::from_iter([
    //     ("nohesi", nohesi_bf_abstr as MOD_T_BF<T, &T>,),
    // ]);
    // let arg1 = T::from(0.0).unwrap();
    // let va2 = res["nohesi"](&arg1, &[&arg1], &vec![T_ARGS::Float(T::from(0.0).unwrap())], &mut BF_VEC::default());
    // res
    MAP_MOD_T_BF::from_iter([
        ("nohesi", nohesi_bf_abstr as MOD_T_BF<T>,),
    ])
}

pub fn map_mod_f<T>() -> MAP_MOD_T<T>
where 
    T: Float,
    T: AddAssign,
{
    MAP_MOD_T::from_iter([
        ("avg", avg_abstr as MOD_T<T>,),
    ])
}

pub fn map_mod_all<T>() -> MAP_MOD_T_BF<T>
where 
    T: Float,
    T: AddAssign,
{
    MAP_MOD_T_BF::from_iter([
        ("nohesi", nohesi_bf_abstr as MOD_T_BF<T>,),
        ("avg", avg_bf_abstr as MOD_T_BF<T>,),
    ])
}

pub fn map_mod_f_from_coll<T>() -> MAP_MOD_T_FROM_COLL<T>
where 
    T: Float,
    T: AddAssign,
{
    MAP_MOD_T_FROM_COLL::from_iter([
        ("nohesi", nohesi_f_abstr as MOD_T_FROM_COLL<T>)
    ])
}

pub fn map_mod_f_from_coll_all<T>() -> MAP_MOD_T_FROM_COLL<T>
where 
    T: Float,
    T: AddAssign,
{
    MAP_MOD_T_FROM_COLL::from_iter([
        ("nohesi", nohesi_f_abstr as MOD_T_FROM_COLL<T>),
        ("avg", avg_from_coll_abstr as MOD_T_FROM_COLL<T>)
    ])
}

pub fn map_mod_coll<C, T>() -> MAP_MOD_COLL<C, T>
where 
    T: Float,
    T: Sum,
    T: AddAssign,
    C: FromIterator<T>,
{
    MAP_MOD_COLL::from_iter([
        ("avg", avg_coll_abstr as MOD_COLL<C, T>,),
        ("nohesi", nohesi_coll_abstr as MOD_COLL<C, T>),
    ])
}