// === CONSTANTS ===
pub const REGISTER_AMOUNT  : usize = 16;
pub const R_DMR_AMOUNT     : usize = 2;

pub const OP_SET : u8 = 0b000_00000;
pub const OP_MOV : u8 = 0b001_00000;
pub const OP_GET : u8 = 0b010_00000;
pub const OP_CMP : u8 = 0b011_00000;
pub const OP_JMP : u8 = 0b100_00000;
pub const OP_JIE : u8 = 0b101_00000;
pub const OP_JIG : u8 = 0b110_00000;
pub const OP_CAL : u8 = 0b111_00000;

pub const DMR_0  : u8 = 0b000_0_0000;
pub const DMR_1  : u8 = 0b000_1_0000;

// Masks
pub const DMR_MSK : u8 = 0b000_1_0000;
pub const OP_MSK  : u8 = 0b111_00000;
pub const CAL_MSK : u8 = 0b000_11111;
pub const REG_MSK : u8 = 0b0000_1111;

// === SYSTEM CALLS ===
pub const C_EXIT : u8 = 0;

// Arithmetics
pub const C_ADD : u8  = 1;
pub const C_SUB : u8  = 2;
pub const C_AND : u8  = 3;
pub const C_OR  : u8  = 4;
pub const C_XOR : u8  = 5;
pub const C_NOT : u8  = 6;
pub const C_INC : u8  = 7;
pub const C_DEC : u8  = 8;

// Program specific
pub const C_PC  : u8  = 9;
pub const C_RET : u8  = 10;

// IO
pub const C_GETI : u8 = 11;
pub const C_PUTI : u8 = 12;


// ERRORS
pub mod error_handling {
    pub const E_EXECUTEABLE_CREATION_FAILURE : &str = "Failed to create executeable file";
    pub const E_WRITE_TO_EXECUTEABLE_FAILURE : &str = "Failed to write into executeable";
    pub const E_COMPILATION_ERROR : &str = "Error occured while compiling";
    pub const E_MODIFYING_ZEROTH_REGISTER_ERROR : &str = "Error while trying to move value into non-modifeable register $0";
    pub const E_UNKNOWN_SYSTEM_CALL_ERROR : &str = "Error while calling an unknown system call";
    pub const E_STANDARD_INPUT_STREAM_READ_FAILURE : &str = "Failed to read from the standard input stream";
    pub const E_READING_EXECUTEABLE_FAILURE : &str = "Failed to read the executeable file.";
    pub const E_WRONG_FILE_TYPE_ERROR : &str = "Wrong file type was passed.";
    pub const E_NO_FILE_PROVIDED : &str = "No file was provided for compilation or emulation.";

    pub fn panic_at_instruction(error_message: &str, program_counter_at_error: usize)
    {
        panic!("{} At instruction: {}", error_message, program_counter_at_error + 1);
    }
}