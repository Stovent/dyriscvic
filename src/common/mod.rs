//! Traits, enums and structs shared betwenn the library's components and that can be useful to the user.

pub mod decoder;
pub mod instruction;
pub mod isa;
pub mod types;

pub use instruction::{Instruction, Instruction32, Instruction64};
use types::*;

/// Returns the width of the instruction word in bytes, 24 if greater than 192 bits.
pub fn get_instruction_length(inst: u16) -> u16 {
    if (inst & 0b11) != 0b11 {
        2
    } else if (inst & 0b1_1111) != 0b1_1111 {
        4
    } else if (inst & 0b11_1111) != 0b01_1111 {
        6
    } else if (inst & 0b111_1111) != 0b011_1111 {
        8
    } else if (inst & 0b0111_0000_0111_1111) != 0b0111_0000_0111_1111 {
        let nnn = inst >> 12 & 0b111;
        10 + 2 * nnn
    } else {
        24
    }
}

/// Returns true if the given number is even, false if it odd.
pub fn is_even<T: Int>(num: T) -> bool {
    return num & 1u16.into() == 0u16.into();
}

/// Converts a 32-bits word to a little-endian slice.
/// `slice[0]` will have the LSB and `slice[3]` the MSB.
pub fn u32_to_slice_le(data: u32) -> [u8; 4] {
    [data as u8, (data >> 8) as u8, (data >> 16) as u8, (data >> 24) as u8]
}

/// Get the integer register ABI name associated with its number.
/// If `abi_name` is true, returns the ABI name as in table 25.1. Otherwise returns `x0`, `x1`, etc.
pub fn get_x_register_name(reg: u8, abi_name: bool) -> String {
    if abi_name {
        String::from(match reg {
            0 => "zero",
            1 => "ra",
            2 => "sp",
            3 => "gp",
            4 => "tp",
            5 => "t0",
            6 => "t1",
            7 => "t2",
            8 => "s0",
            9 => "s1",
            10 => "a0",
            11 => "a1",
            12 => "a2",
            13 => "a3",
            14 => "a4",
            15 => "a5",
            16 => "a6",
            17 => "a7",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "s8",
            25 => "s9",
            26 => "s10",
            27 => "s11",
            28 => "t3",
            29 => "t4",
            30 => "t5",
            31 => "t6",
            _ => "Unknown register"
        })
    } else {
        format!("x{}", reg)
    }
}

/// Get the floating point register ABI name associated with its number.
/// If `abi_name` is true, returns the ABI name as in table 25.1. Otherwise returns `f0`, `f1`, etc.
pub fn get_f_register_name(reg: u8, abi_name: bool) -> String {
    if abi_name {
        String::from(match reg {
            0 => "ft0",
            1 => "ft1",
            2 => "ft2",
            3 => "ft3",
            4 => "ft4",
            5 => "ft5",
            6 => "ft6",
            7 => "ft7",
            8 => "fs0",
            9 => "fs1",
            10 => "fa0",
            11 => "fa1",
            12 => "fa2",
            13 => "fa3",
            14 => "fa4",
            15 => "fa5",
            16 => "fa6",
            17 => "fa7",
            18 => "fs2",
            19 => "fs3",
            20 => "fs4",
            21 => "fs5",
            22 => "fs6",
            23 => "fs7",
            24 => "fs8",
            25 => "fs9",
            26 => "fs10",
            27 => "fs11",
            28 => "ft8",
            29 => "ft9",
            30 => "ft10",
            31 => "ft11",
            _ => "Unknown register"
        })
    } else {
        format!("f{}", reg)
    }
}
