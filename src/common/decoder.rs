use crate::common::{instruction::*, isa::*};

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

pub trait Format {
}

impl Format for Isa {}

impl Isa {
    /// Returns the Isa corresponding to the given 32-bits opcode.
    pub fn from_opcode_32(opcode: u32) -> Isa {
        match opcode & 0b111_1111 {
            0b000_0011 => Self::I_LOAD[opcode as usize >> 12 & 0b111],
            0b000_1111 => Isa::FENCE,
            0b001_0011 => Self::get_immediate_32(opcode),
            0b001_0111 => Isa::AUIPC,
            0b001_1011 => Self::get_immediate_64(opcode),
            0b010_0011 => Self::I_STORE[opcode as usize >> 12 & 0b11],
            0b011_0011 => Self::get_arithmetic_32(opcode),
            0b011_0111 => Isa::LUI,
            0b011_1011 => Self::get_arithmetic_64(opcode),
            0b110_0011 => Self::I32_BRANCH[opcode as usize >> 12 & 0b111],
            0b110_0111 => Isa::JALR,
            0b110_1111 => Isa::JAL,
            0b111_0011 => if opcode == 0x0000_0073 { Isa::ECALL } else if opcode == 0x0010_0073 { Isa::EBREAK } else { Isa::UNKNOWN },
            _ => Isa::UNKNOWN,
        }
    }

    pub const FORMAT: [fn(Isa, u64, u32) -> Instruction; Isa::_SIZE as usize] = [
        Instruction::empty,

        Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_u, Instruction::decode_type_b, Instruction::decode_type_b, Instruction::decode_type_b,
        Instruction::decode_type_b, Instruction::decode_type_b, Instruction::decode_type_b, Instruction::empty,         Instruction::empty,         Instruction::decode_type_i, Instruction::decode_type_j, Instruction::decode_type_i,
        Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_u, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i,
        Instruction::decode_type_s, Instruction::decode_type_s, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_r,
        Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_s, Instruction::decode_type_r, Instruction::decode_type_i,

        Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_s, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i,
        Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_r,
    ];

    const I32_ARITHMETIC: [Isa; 8] = [Isa::ADD, Isa::SLL, Isa::SLT, Isa::SLTU, Isa::XOR, Isa::SRL, Isa::OR, Isa::AND];
    const I32_BRANCH: [Isa; 8] = [Isa::BEQ, Isa::BNE, Isa::UNKNOWN, Isa::UNKNOWN, Isa::BLT, Isa::BGE, Isa::BLTU, Isa::BGEU];
    const I32_IMMEDIATE: [Isa; 8] = [Isa::ADDI, Isa::SLLI, Isa::SLTI, Isa::SLTIU, Isa::XORI, Isa::SRLI, Isa::ORI, Isa::ANDI];
    const I_LOAD: [Isa; 8] = [Isa::LB, Isa::LH, Isa::LW, Isa::LD, Isa::LBU, Isa::LHU, Isa::LWU, Isa::UNKNOWN];
    const I_STORE: [Isa; 4] = [Isa::SB, Isa::SH, Isa::SW, Isa::SD];

    const I64_IMMEDIATE: [Isa; 8] = [Isa::ADDIW, Isa::SLLIW, Isa::UNKNOWN, Isa::UNKNOWN, Isa::UNKNOWN, Isa::SRLIW, Isa::UNKNOWN, Isa::UNKNOWN];
    const I64_ARITHMETIC: [Isa; 8] = [Isa::ADDW, Isa::SLLW, Isa::UNKNOWN, Isa::UNKNOWN, Isa::UNKNOWN, Isa::SRLW, Isa::UNKNOWN, Isa::UNKNOWN];

    fn get_immediate_32(opcode: u32) -> Isa {
        if opcode & 0xFC00_707F == 0x4000_5013 {
            Isa::SRAI
        } else {
            Self::I32_IMMEDIATE[opcode as usize >> 12 & 0b111]
        }
    }

    fn get_immediate_64(opcode: u32) -> Isa {
        if opcode & 0xFE00_707F == 0x4000_501B {
            Isa::SRAIW
        } else {
            Self::I64_IMMEDIATE[opcode as usize >> 12 & 0b111]
        }
    }

    fn get_arithmetic_32(opcode: u32) -> Isa {
        if opcode & 0xFE00_707F == 0x4000_0033 {
            Isa::SUB
        } else if opcode & 0xFE00_707F == 0x4000_5033 {
            Isa::SRA
        } else if opcode & 0xFE00_0000 != 0 {
            Isa::UNKNOWN
        } else {
            Self::I32_ARITHMETIC[opcode as usize >> 12 & 0b111]
        }
    }

    fn get_arithmetic_64(opcode: u32) -> Isa {
        if opcode & 0xFE00_707F == 0x4000_003B {
            Isa::SUBW
        } else if opcode & 0xFE00_707F == 0x4000_503B {
            Isa::SRAW
        } else if opcode & 0xFE00_0000 != 0 {
            Isa::UNKNOWN
        } else {
            Self::I64_ARITHMETIC[opcode as usize >> 12 & 0b111]
        }
    }
}
