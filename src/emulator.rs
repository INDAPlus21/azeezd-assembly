use std::fs;

use super::consts;

pub fn emulate(filename: String) {
    // Set up registers
    let mut R_DMR : [u32; consts::R_DMR_AMOUNT] = [0; consts::R_DMR_AMOUNT];
    let mut REGISTERS : [u32; consts::REGISTER_AMOUNT] = [0; consts::REGISTER_AMOUNT];

    // Assembler Data
    let mut program_counter : usize = 0;
    let instructions = fs::read(filename).expect(consts::error_handling::E_READING_EXECUTEABLE_FAILURE);
    while instructions[program_counter] != (consts::OP_CAL | consts::C_EXIT) {
        let instruction = instructions[program_counter];
        match get_operation(&instruction) {
            consts::OP_SET => {
                R_DMR[get_dmr(&instruction) as usize] = get_register(&instruction) as u32; // register param is the same as immediate for set operation
            },

            consts::OP_MOV => {
                let target_register = get_register(&instruction);
                if target_register != 0 {
                    REGISTERS[target_register as usize] = R_DMR[get_dmr(&instruction) as usize];
                }
            },

            consts::OP_GET => {
                R_DMR[get_dmr(&instruction) as usize] = REGISTERS[get_register(&instruction) as usize];
            },

            consts::OP_CMP => {
                REGISTERS[14] = (R_DMR[get_dmr(&instruction) as usize] > REGISTERS[get_register(&instruction) as usize]) as u32; 
                REGISTERS[15] = (R_DMR[get_dmr(&instruction) as usize] < REGISTERS[get_register(&instruction) as usize]) as u32;
            },

            consts::OP_JMP => {
                REGISTERS[13] = program_counter as u32;
                program_counter = (program_counter as isize + calculate_jump(&instruction)) as usize;
                continue;
            },

            consts::OP_JIE => {
                if REGISTERS[14] == 0 && REGISTERS[15] == REGISTERS[14] {
                    program_counter = (program_counter as isize + calculate_jump(&instruction)) as usize;
                    continue;
                }
            },

            consts::OP_JIG => {
                if REGISTERS[14] == 1 && REGISTERS[15] == 0 {
                    program_counter = (program_counter as isize + calculate_jump(&instruction)) as usize;
                    continue;
                }
            },

            consts::OP_CAL => {
                match get_call_code(&instruction) {
                    consts::C_ADD  => {REGISTERS[11] = R_DMR[0] + R_DMR[1];},
                    consts::C_SUB  => {REGISTERS[11] = R_DMR[0] - R_DMR[1];},
                    consts::C_AND  => {REGISTERS[11] = R_DMR[0] & R_DMR[1];},
                    consts::C_OR   => {REGISTERS[11] = R_DMR[0] | R_DMR[1];}, 
                    consts::C_XOR  => {REGISTERS[11] = R_DMR[0] ^ R_DMR[1];},
                    consts::C_NOT  => {REGISTERS[11] = !R_DMR[0];},
                    consts::C_INC  => {REGISTERS[11] = R_DMR[0] + 1;},
                    consts::C_DEC  => {REGISTERS[11] = R_DMR[0] - 1;},
                    consts::C_PC   => {REGISTERS[13] = program_counter as u32;},
                    consts::C_RET  => {program_counter = REGISTERS[13] as usize;},
                    consts::C_GETI => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect(consts::error_handling::E_STANDARD_INPUT_STREAM_READ_FAILURE);
                        REGISTERS[11] = input.trim().parse::<u32>().unwrap();
                    },
                    consts::C_PUTI => {print!("{}", REGISTERS[12])}
                    _ => {}
                }
            }
            _ => {}
        }
        program_counter += 1;
    }
}

/// == Get appropiate bits through masking

fn get_operation(instruction: &u8) -> u8 {
    instruction & consts::OP_MSK
}

fn get_dmr(instruction: &u8) -> u8 {
    (instruction & consts::DMR_MSK) >> 4
}

fn get_register(instruction: &u8) -> u8 {
    instruction & consts::REG_MSK
}

fn get_call_code(instruction: &u8) -> u8 {
    instruction & consts::CAL_MSK
}

fn calculate_jump(instruction: &u8) -> isize {
    let destination : isize = (instruction & 0b1111) as isize;
    if instruction & 0b000_1_0000 == 0b000_1_0000 {
         -destination
    }
    else {
        destination
    }
}
