use std::collections::HashMap;

use crate::eval::object::Object;

pub mod array;
pub mod util;

pub fn get_std_lib(lib: String) -> Option<HashMap<String, Object>> {
    match lib.as_str() {
        "std:util" => Some(util::add_globals()),
        "std:array" => Some(array::add_globals()),
        _ => None,
    }
}
