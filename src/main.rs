pub mod ast;
pub mod lexer;
pub mod parser;
// pub mod std_lib;
pub mod token;
pub mod eval;

use lexer::Lexer;
use parser::Parser;
use std::{cell::RefCell, env, fs, rc::Rc};
use eval::{object::Object, store::Store, Eval};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 && args[1].as_str() == "run" {
        let filename = &args[2].split('.').collect::<Vec<_>>();
        if filename[filename.len() - 1] != "ext" {
            println!("File must be .ext");
            return;
        }
        let content = fs::read_to_string(&args[2]).expect("Could not read file.");

        let store = Store::new();
        let mut evaluator = Eval {
            store: Rc::new(RefCell::new(store)),
        };
        let lexer = Lexer::new(content);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if !parser.errors.is_empty() {
            for e in parser.errors.iter() {
                println!("\t{}", e);
            }
            return;
        }
        let res = evaluator.eval(program);

        if let Some(o) = res {
            match o {
                Object::Null => (),
                _ => println!("{}", o),
            }
        }
        return;
    }
}
