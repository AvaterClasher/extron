#[cfg(target_os = "emscripten")]
extern crate extron;

#[cfg(not(target_os = "emscripten"))]
use clap::{Arg, Command};

#[cfg(not(target_os = "emscripten"))]
use std::fs;

#[cfg(target_os = "emscripten")]
use std::{ffi::CString, mem, os::raw::c_char};

#[cfg(not(target_os = "emscripten"))]
fn main() {
    let matches = Command::new("Extron")
        .version("1.0")
        .author("Soumyadip Moni <avater.clasher@gmail.com>")
        .about("Interprets .ext files or starts a REPL")
        .subcommand(
            Command::new("run")
                .about("Runs a specified .ext file")
                .arg(Arg::new("file")
                    .help("The .ext file to run")
                    .required(true)
                    .index(1)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        if let Some(file) = matches.get_one::<String>("file") {
            let filename = file.split('.').collect::<Vec<_>>();
            if filename.last() != Some(&"ext") {
                println!("File must have the extension .ext");
                return;
            }
            let content = fs::read_to_string(file).expect("Could not read file.");
            extron::interpret(content.as_str());
        }
    } else {
        println!("Welcome to the Extron REPL. Type in commands to get started.");
        extron::repl::start();
    }
}

#[cfg(target_os = "emscripten")]
fn main() {}

#[cfg(target_os = "emscripten")]
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn interpret(src: *mut c_char) -> usize {
    let code = CString::from_raw(src).to_str().unwrap().to_string();
    let _ = extron::interpret(&code);
    0
}

#[cfg(target_os = "emscripten")]
#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buf: Vec<u8> = Vec::with_capacity(len);

    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

#[cfg(target_os = "emscripten")]
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    let data = Vec::from_raw_parts(ptr, len, len);

    mem::drop(data)
}
