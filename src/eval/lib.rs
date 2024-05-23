use super::{object::*, store::Store, Eval};
use crate::std_lib::get_std_lib;
use crate::{lexer::Lexer, parser::Parser};
use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

pub fn load_ext(lib: String) -> Option<HashMap<String, Object>> {
    if lib.starts_with("std:") {
        return get_std_lib(lib);
    }
    let filename = format!("./{}.ext", lib);
    let file = fs::read_to_string(filename).expect("Lib not found.");
    let mut parser = Parser::new(Lexer::new(file));
    let program = parser.parse_program();
    if !parser.errors.is_empty() {
        for e in parser.errors.iter() {
            println!("\t{}", e);
        }
        return None;
    };
    let mut eval = Eval::new(Rc::new(RefCell::new(Store::new())));
    eval.eval(program);

    let store = (*eval.store.borrow()).to_owned().store;
    let mut final_env = HashMap::new();
    for (k, v) in store.iter() {
        final_env.insert(k.clone(), v.clone());
    }
    Some(final_env)
}
