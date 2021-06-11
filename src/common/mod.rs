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

pub fn is_even<T: Int>(num: T) -> bool {
    return num & 1u16.into() == 0u16.into();
}

pub fn u32_to_slice_le(data: u32) -> [u8; 4] {
    [data as u8, (data >> 8) as u8, (data >> 16) as u8, (data >> 24) as u8]
}
