mod compiler;
mod consts;
mod emulator;

use std::io::prelude::*;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    match args[1].to_string().as_str() {
        "r" => {emulator::emulate(args[2].to_string());},
        "c" => {compiler::compile(args[2].to_string());}
        "cr" => {
            compiler::compile(args[2].to_string());
            emulator::emulate("o.vivex".to_string())
        }
        _ => {}
    }
}
