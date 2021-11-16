use super::consts;
use std::fs;
use std::fs::File;
use std::collections::{HashMap};
use std::io::prelude::*;

pub fn compile(filename: String) {
    let mut program_counter = 0;
    let mut labels : HashMap<String, u32> = HashMap::new();
    let mut instruction_bits : Vec<u8> = Vec::new();
    match fs::read_to_string(filename)
    {
        Ok(_text) => {
            let instructions : Vec<String> = _text
                                                .lines()
                                                .map(|_line| _line.to_string())
                                                .collect();

            for inst in instructions {
                let mut _instruction_as_byte : u8 = 0;
                
                let tokens : Vec<String> = inst
                                .split_whitespace()
                                .map(|_token| _token.to_string())
                                .collect();
                match tokens.len() {
                    1 => { // LABELS
                        labels.insert(tokens[0][0..tokens[0].len()-1].to_string(), program_counter);
                        continue;
                    },
                    3 => { // MANIPULATION INSTRUCTIONS
                        match tokens[0].as_str() {
                            "set" => {
                                _instruction_as_byte |= consts::OP_SET;
                                _instruction_as_byte |= tokens[2].parse::<u8>().unwrap();
                            },
                            "mov" => {_instruction_as_byte |= consts::OP_MOV;},
                            "get" => {_instruction_as_byte |= consts::OP_GET;},
                            "cmp" => {_instruction_as_byte |= consts::OP_CMP;},
                            _ => {}
                        }
                        match tokens[1].as_str() {
                            "#0" => {_instruction_as_byte |= consts::DMR_0;}
                            "#1" => {_instruction_as_byte |= consts::DMR_1;}
                            _=> {}
                        }
                        if tokens[0] != "set"
                        {
                            _instruction_as_byte |= tokens[2][1..]
                                                        .parse::<u8>().unwrap();
                        }
                    },
                    2 => { // if 2: BRANCHING INSTRUCTIONS
                        match tokens[0].as_str() {
                            "jmp" => {_instruction_as_byte |= consts::OP_JMP;},
                            "jie" => {_instruction_as_byte |= consts::OP_JIE;},
                            "jig" => {_instruction_as_byte |= consts::OP_JIG;},
                            "cal" => {
                                _instruction_as_byte |= consts::OP_CAL;
                                _instruction_as_byte |= tokens[1].parse::<u8>().unwrap();
                            },
                            _ => {}
                        }

                        if tokens[0] != "cal" { // IF any jump instruction
                            // Calculate step difference and store it as 5 bits in the instruction
                            let jump_size  = (*labels.get(&tokens[1]).unwrap() as i32) - program_counter as i32;
                            if jump_size < 0 {
                                _instruction_as_byte |= 0b000_1_0000
                            }
                            _instruction_as_byte |= jump_size.abs() as u8;
                        }
                    }
                    _ => {}
                }

                program_counter += 1;
                instruction_bits.push(_instruction_as_byte);
            }
            instruction_bits.push(consts::OP_CAL | consts::C_EXIT); // TERMINATE PROG INSTRUCTION (Added for safety and control)

            let mut out_file = File::create(format!("o.vivex")).expect("Failed to create executeable file!");
            out_file.write_all(&instruction_bits).expect("Error while writing to execulteable file");
        }
        Err(_error) => {
            println!("Compilation Error!");
        }
    }                                      
}
