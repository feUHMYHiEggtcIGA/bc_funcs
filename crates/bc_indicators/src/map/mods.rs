use std::ops::AddAssign;
use std::iter::Sum;

use num_traits::Float;
use bc_utils_lg::structs_and_types::funcs_abstr::*;
use bc_utils_lg::structs_and_types::maps_abstr::*;

use crate::mods::{
    avg::{avg_abstr, avg_coll_abstr, avg_from_coll_abstr, avg_bf_abstr},
    nohesi::{nohesi_bf_abstr, nohesi_coll_abstr, nohesi_f_abstr},
};


pub fn map_mod_bf<T>() -> MAP_MOD_T_BF<T, T>
where 
    T: Float,
    T: AddAssign,
{
    MAP_MOD_T_BF::from_iter([
        ("nohesi", nohesi_bf_abstr as MOD_T_BF<T, T>,),
    ])
}

pub fn map_mod_f<T>() -> MAP_MOD_T<T, T>
where 
    T: Float,
    T: AddAssign,
{
    MAP_MOD_T::from_iter([
        ("avg", avg_abstr as MOD_T<T, T>,),
    ])
}

pub fn map_mod_all<T>() -> MAP_MOD_T_BF<T, T>
where 
    T: Float,
    T: AddAssign,
{
    MAP_MOD_T_BF::from_iter([
        ("nohesi", nohesi_bf_abstr as MOD_T_BF<T, T>,),
        ("avg", avg_bf_abstr as MOD_T_BF<T, T>,),
    ])
}

pub fn map_mod_f_from_coll<T>() -> MAP_MOD_T_FROM_COLL<T, T>
where 
    T: Float,
    T: AddAssign,
{
    MAP_MOD_T_FROM_COLL::from_iter([
        ("nohesi", nohesi_f_abstr as MOD_T_FROM_COLL<T, T>)
    ])
}

pub fn map_mod_f_from_coll_all<T>() -> MAP_MOD_T_FROM_COLL<T, T>
where 
    T: Float,
    T: AddAssign,
{
    MAP_MOD_T_FROM_COLL::from_iter([
        ("nohesi", nohesi_f_abstr as MOD_T_FROM_COLL<T, T>),
        ("avg", avg_from_coll_abstr as MOD_T_FROM_COLL<T, T>)
    ])
}

pub fn map_mod_coll<C, T>() -> MAP_MOD_COLL<C, T, T>
where 
    T: Float,
    T: Sum,
    T: AddAssign,
    C: FromIterator<T>,
{
    MAP_MOD_COLL::from_iter([
        ("avg", avg_coll_abstr as MOD_COLL<C, T, T>,),
        ("nohesi", nohesi_coll_abstr as MOD_COLL<C, T, T>),
    ])
}