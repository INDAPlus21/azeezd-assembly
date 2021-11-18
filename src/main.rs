mod compiler;
mod consts;
mod emulator;

fn main() {
    let args : Vec<String> = std::env::args().collect();

    // Insufficient amount of arguments
    if args.len() < 3 {
        panic!("{}", consts::error_handling::E_NO_FILE_PROVIDED);
    }

    // Output file name was provided, prepare optional to be passed in compiler
    let mut output = None;
    if args.len() > 3 {
        let mut name = args[3].to_string();

        // Adds .vivex suffix if not provided
        if !name.ends_with(".vivex") {name.push_str(".vivex")}

        output = Some(name);
    }

    // Wrong file type was passed into compiler/emulator
    if !args[2].ends_with(".viv") & !args[2].ends_with(".vivex") {
        panic!("{} File: {}", consts::error_handling::E_WRONG_FILE_TYPE_ERROR, args[2]);
    }

    match args[1].to_string().as_str() {
        "r" => {emulator::emulate(args[2].to_string());}, // RUN

        "c" => {compiler::compile(args[2].to_string(), &output);}, // COMPILE

        // COMPILE & RUN
        "cr" => {
            compiler::compile(args[2].to_string(), &output);
            emulator::emulate(
                match output {
                    None => "out.vivex".to_string(), // default output file is out.vivex
                    Some(p) => p
                }
            );
        }
        _ => {}
    }
}
