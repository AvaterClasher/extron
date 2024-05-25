use std::collections::HashMap;
use std::io::Write;

use crate::eval::object::Object;

use super::Res;

pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("length"), Object::Inbuilt(length));
    globals.insert(String::from("input"), Object::Inbuilt(input));
    globals.insert(String::from("sleep"), Object::Inbuilt(sleep));
    return Res { globals, raw: None };
}

pub fn length(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }
    match &args[0] {
        Object::String(s) => Object::Number(s.len() as f64),
        Object::Array(a) => Object::Number(a.len() as f64),
        o => Object::Error(format!("Argument must be a string or array. Got {}", o)),
    }
}

pub fn input(args: Vec<Object>) -> Object {
    print!("{}", &args[0]);
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim_end().to_string();
    return Object::String(input);
}

pub fn sleep(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }
    if let Object::Number(n) = &args[0] {
        std::thread::sleep(std::time::Duration::from_millis(*n as u64));
    }
    Object::Null
}
