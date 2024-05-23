use std::collections::HashMap;

use crate::eval::object::Object;

pub fn add_globals() -> HashMap<String, Object> {
    let mut globals = HashMap::new();
    globals.insert(String::from("push"), Object::Inbuilt(push));
    globals
}

pub fn push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "Wrong number of arguments. got = {}, Expected = 2",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(a) => {
            let mut array = a.clone();
            array.push(args[1].clone());
            Object::Array(array)
        }
        o => Object::Error(format!("First argument must be an array. Got {}", o)),
    }
}
