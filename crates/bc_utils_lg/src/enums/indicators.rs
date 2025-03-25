use rustc_hash::FxHashMap;


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum T_HASHMAP {
    Float64(FxHashMap<&'static str, f64>),
    Float64_r(FxHashMap<&'static str, &'static f64>),
    VecF64(FxHashMap<&'static str, Vec<f64>>),
    VecF64_r(FxHashMap<&'static str, Vec<&'static f64>>),
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum T_ARGS {
    Float32(f32),
    Float64(f64),
    Usize(usize),
    String(String),
    None(())
}

impl T_ARGS {
    pub fn unwrap_f32<'a>(&'a self) -> &'a f32 {
        match &self {
            T_ARGS::Float32(v) => v,
            _ => panic!("unwrap failed"),
        }
    } 
    
    pub fn unwrap_f64<'a>(&'a self) -> &'a f64 {
        match self {
            T_ARGS::Float64(v) => v,
            _ => panic!("unwrap failed"),
        }
    }

    pub fn unwrap_usize<'a>(&'a self) -> &'a usize {
        match self {
            T_ARGS::Usize(v) => v,
            _ => panic!("unwrap failed"),
        }
    }
    
    pub fn unwrap_string<'a>(&'a self) -> &'a String {
        match self {
            T_ARGS::String(v) => v,
            _ => panic!("unwrap failed"),
        }
    } 
}