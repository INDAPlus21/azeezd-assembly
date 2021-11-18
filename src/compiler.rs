use super::consts;
use std::fs;
use std::fs::File;
use std::collections::{HashMap};
use std::io::prelude::*;

pub fn compile(filename: String, output: &Option<String>) {

    let mut program_counter = 0;
    let mut labels : HashMap<String, usize> = HashMap::new();

    // Instructions stored as bits (will be saved to output file)
    let mut instruction_bits : Vec<u8> = Vec::new();

    match fs::read_to_string(filename)
    {
        Ok(_text) => {
            // Get instructions as strings and filter out empty lines
            let instructions : Vec<String> = _text
                                                .lines()
                                                .map(|_line| _line.trim().to_string())
                                                .filter(|_line| _line.len() > 0)
                                                .collect();

            // Get labels and store their position to the hashmap
            let mut instruction_shift = 0;
            for instr_idx in 0..instructions.len() {
                let _line = &instructions[instr_idx];

                // All labels end with :
                if _line.ends_with(':') {
                    // Every labels shifts the program counter by 1 in the opposite direction
                    // Removing that error to write correctly in the output file and for correct jumping
                    labels.insert(_line[.._line.len()-1].to_string(), instr_idx - instruction_shift);
                    instruction_shift += 1; 
                }
            }

            for inst in instructions {
                let mut instruction_as_byte : u8 = 0;
                
                // Tokens in an instruction are separated by a space
                let tokens : Vec<String> = inst
                                .split_whitespace()
                                .map(|_token| _token.to_string())
                                .collect();

                match tokens.len() {
                    1 => { // 1 token = LABELS
                        continue; // Labels are not compiled to output file
                    },
                    3 => { // 3 tokens = MANIPULATION INSTRUCTIONS
                        match tokens[0].as_str() {
                            "set" => {
                                instruction_as_byte |= consts::OP_SET;
                                instruction_as_byte |= tokens[2].parse::<u8>().unwrap(); // Get immediate
                            },
                            "mov" => {instruction_as_byte |= consts::OP_MOV;},
                            "get" => {instruction_as_byte |= consts::OP_GET;},
                            "cmp" => {instruction_as_byte |= consts::OP_CMP;},
                            _ => {
                                consts::error_handling::panic_at_instruction(consts::error_handling::E_UNKNOWN_OPERATION, program_counter);
                            }
                        }
                        match tokens[1].as_str() {
                            "#0" => {instruction_as_byte |= consts::DMR_0;}
                            "#1" => {instruction_as_byte |= consts::DMR_1;}
                            _=> {
                                consts::error_handling::panic_at_instruction(consts::error_handling::E_UNKNOWN_DESTINATION_DMR, program_counter);
                            }
                        }

                        if tokens[0] != "set"
                        {
                            // Get IMR destination or Immediate
                            let value = tokens[2][1..].parse::<u8>().unwrap();
                            
                            if value as usize >= consts::REGISTER_AMOUNT {
                                consts::error_handling::panic_at_instruction(consts::error_handling::E_UNKNOWN_DESTINATION_IMR, program_counter);
                            }

                            // Overwriting zeroth register
                            if tokens[0] == "mov" && value == 0 {
                                consts::error_handling::panic_at_instruction(consts::error_handling::E_MODIFYING_ZEROTH_REGISTER_ERROR, program_counter);
                            }
                            instruction_as_byte |= value;
                        }
                    },
                    2 => { // 2 tokens = BRANCHING INSTRUCTIONS
                        match tokens[0].as_str() {
                            "jmp" => {instruction_as_byte |= consts::OP_JMP;},
                            "jie" => {instruction_as_byte |= consts::OP_JIE;},
                            "jig" => {instruction_as_byte |= consts::OP_JIG;},
                            "cal" => {
                                instruction_as_byte |= consts::OP_CAL;
                                
                                // Get syscall code
                                let code = tokens[1].parse::<u8>().unwrap();

                                if check_call_existance(code) {
                                    instruction_as_byte |= code;
                                }
                                else {
                                    consts::error_handling::panic_at_instruction(consts::error_handling::E_UNKNOWN_SYSTEM_CALL_ERROR, program_counter);
                                }
                            },
                            _ => {
                                consts::error_handling::panic_at_instruction(consts::error_handling::E_UNKNOWN_OPERATION, program_counter);
                            }
                        }

                        if tokens[0] != "cal" { // IF any jump instruction
                            // Calculate step difference and store it as 5 bits in the instruction
                            let jump_size  = (*labels.get(&tokens[1]).unwrap() as i32) - program_counter as i32;

                            // Negative jumps are signed by the fifth bit
                            if jump_size < 0 {
                                instruction_as_byte |= 0b000_1_0000
                            }
                            instruction_as_byte |= jump_size.abs() as u8;
                        }
                    }
                    _ => {
                        consts::error_handling::panic_at_instruction(consts::error_handling::E_WRONG_FORMAT_OF_INSTRUCTION, program_counter);
                    }
                }

                program_counter += 1;
                instruction_bits.push(instruction_as_byte);
            }
            instruction_bits.push(consts::OP_CAL | consts::C_EXIT); // TERMINATE PROG INSTRUCTION (Added for safety and control)

            // Create output file and write instruction bytes to it
            let mut out_file = File::create(
                match output {
                    None => "out.vivex".to_string(),
                    Some(p) => p.to_string()
                }
            ).expect(consts::error_handling::E_EXECUTEABLE_CREATION_FAILURE);

            out_file.write_all(&instruction_bits).expect(consts::error_handling::E_WRITE_TO_EXECUTEABLE_FAILURE);
        }
        Err(_error) => {
            panic!("{}", consts::error_handling::E_COMPILATION_ERROR);
        }
    }                                      
}

/// Checks if the sys call code passed exists
fn check_call_existance(code: u8) -> bool {
    code < 13 // There are 13 codes as of now
}
