mod compiler;
mod consts;
mod emulator;

fn main() {
    let args : Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        panic!("{}", consts::error_handling::E_NO_FILE_PROVIDED);
    }

    let mut output = None;
    if args.len() > 3 {
        let mut name = args[3].to_string();
        if !name.ends_with(".vivex") {name.push_str(".vivex")}
        output = Some(name);
    }

    if !args[2].ends_with(".viv") | !args[2].ends_with(".vivex") {
        panic!("{} File: {}", consts::error_handling::E_WRONG_FILE_TYPE_ERROR, args[2]);
    }

    match args[1].to_string().as_str() {
        "r" => {emulator::emulate(args[2].to_string());},

        "c" => {
            
            compiler::compile(args[2].to_string(), &output);
        },

        "cr" => {
            compiler::compile(args[2].to_string(), &output);
            emulator::emulate(
                match output {
                    None => "out.vivex".to_string(),
                    Some(p) => p
                }
            );
        }
        _ => {}
    }
}
