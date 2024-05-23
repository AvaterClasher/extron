use std::collections::HashMap;

use crate::eval::object::Object;

pub mod util;
pub mod array;
pub mod fs;

pub struct Res {
    pub globals: HashMap<String, Object>,
    pub raw: Option<String>,
}
pub fn get_std_lib(lib: String) -> Option<Res> {
    match lib.as_str() {
        "std:util" => Some(util::add_globals()),
        "std:array" => Some(array::add_globals()),
        "std:fs" => Some(fs::add_globals()),
        _ => None,
    }
}