use num_traits::Float;
use rustc_hash::FxHashMap;


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum T_HASHMAP<T> 
where  
    T: Float
{ 
    Float(FxHashMap<&'static str, T>),
    VecF(FxHashMap<&'static str, Vec<T>>),
}

impl<T> T_HASHMAP<T>
where  
    T: Float
{
    pub fn unwrap_f(&mut self) -> &mut FxHashMap<&'static str, T> {
        match self {
            T_HASHMAP::Float(v) => v,
            _ => panic!("unwrap failed"),
        }
    }

    pub fn unwrap_vec_f(&mut self) -> &mut FxHashMap<&'static str, Vec<T>> {
        match self {
            T_HASHMAP::VecF(v) => v,
            _ => panic!("unwrap failed"),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum T_ARGS<'a, T>
where  
    T: Float
{
    Float(T),
    Usize(usize),
    String(String),
    Str(&'a str),
    None(()),
}

impl<'a, T> T_ARGS<'a, T>
where  
    T: Float
{
    pub fn unwrap_f(&self) -> & T {
        match self {
            T_ARGS::Float(v) => v,
            _ => panic!("unwrap failed"),
        }
    }

    pub fn unwrap_usize(&self) -> & usize {
        match self {
            T_ARGS::Usize(v) => v,
            _ => panic!("unwrap failed"),
        }
    }
    
    pub fn unwrap_string(&self) -> & String {
        match self {
            T_ARGS::String(v) => v,
            _ => panic!("unwrap failed"),
        }
    }

    pub fn unwrap_str(&self) -> &'a str {
        match self {
            T_ARGS::Str(v) => v,
            _ => panic!("unwrap failed"),
        }
    }
}