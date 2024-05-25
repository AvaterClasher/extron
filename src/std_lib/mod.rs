use std::collections::HashMap;

use crate::eval::object::Object;

pub mod array;
pub mod fs;
pub mod math;
pub mod string;
pub mod util;

pub struct Res {
    pub globals: HashMap<String, Object>,
    pub raw: Option<String>,
}
pub fn get_std_lib(lib: String) -> Option<Res> {
    match lib.as_str() {
        "std:util" => Some(util::add_globals()),
        "std:array" => Some(array::add_globals()),
        "std:fs" => Some(fs::add_globals()),
        "std:string" => Some(string::add_globals()),
        "std:math" => Some(math::add_globals()),
        _ => None,
    }
}
