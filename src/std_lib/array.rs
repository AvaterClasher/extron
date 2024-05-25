use std::collections::HashMap;

use crate::eval::object::Object;

use super::Res;

pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("pop"), Object::Inbuilt(pop));
    globals.insert(String::from("head"), Object::Inbuilt(head));
    globals.insert(String::from("tail"), Object::Inbuilt(tail));
    globals.insert(String::from("push"), Object::Inbuilt(push));
    globals.insert(String::from("import"), Object::Inbuilt(import));
    return Res {
        globals,
        raw: Some(
            "
    import \"std:util\";

    let map = fn (arr, f) {
        let res = [];
        let iter = fn (array) {
            if (length(array) == 0) {
                return;
            } else {
                set res = push(res, f(array[0]));
                iter(tail(array));
            }
        };
    iter(arr)
    return res;
    }; 
"
            .to_string(),
        ),
    };
}

pub fn push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 2.",
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

pub fn pop(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(a) => {
            let mut array = a.clone();
            array.pop();
            Object::Array(array)
        }
        o => Object::Error(format!("First argument must be an array. Got {}", o)),
    }
}

pub fn head(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(a) => {
            let mut array = a.clone();
            array.pop();
            Object::Array(array)
        }
        o => Object::Error(format!("First argument must be an array. Got {}", o)),
    }
}

pub fn tail(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(a) => Object::Array(a[1..].to_vec()),
        o => Object::Error(format!("First argument must be an array. Got {}", o)),
    }
}

pub fn import(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 2.",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(a) => {
            let array = a.clone();
            Object::Bool(array.contains(&args[1]))
        }
        o => Object::Error(format!("First argument must be an array. Got {}", o)),
    }
}
