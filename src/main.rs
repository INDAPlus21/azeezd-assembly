mod compiler;
mod consts;
mod emulator;

use std::io::prelude::*;

fn main() {
    let args : Vec<String> = std::env::args().collect();

    compiler::compile(args[1].to_string());
}
