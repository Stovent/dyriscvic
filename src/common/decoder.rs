use crate::common::{instruction::*, isa::*, types::*};

pub trait Format<U: Unsigned<S>, S: Signed<U>> {
    const FORMAT: [fn(ISA, U, u32) -> Instruction<U, S>; ISA::_SIZE as usize] = [
        Instruction::empty,

        Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_u, Instruction::decode_type_b, Instruction::decode_type_b, Instruction::decode_type_b,
        Instruction::decode_type_b, Instruction::decode_type_b, Instruction::decode_type_b, Instruction::empty,         Instruction::empty,         Instruction::decode_type_i, Instruction::decode_type_j, Instruction::decode_type_i,
        Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_u, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i,
        Instruction::decode_type_s, Instruction::decode_type_s, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_r,
        Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_s, Instruction::decode_type_r, Instruction::decode_type_i,

        Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_s, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i,
        Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_r,
    ];
}

impl<U: Unsigned<S>, S: Signed<U>> Format<U, S> for ISA {}

impl ISA {
    const I32_ARITHMETIC: [ISA; 8] = [ISA::ADD, ISA::SLL, ISA::SLT, ISA::SLTU, ISA::XOR, ISA::SRL, ISA::OR, ISA::AND];
    const I32_BRANCH: [ISA; 8] = [ISA::BEQ, ISA::BNE, ISA::UNKNOWN, ISA::UNKNOWN, ISA::BLT, ISA::BGE, ISA::BLTU, ISA::BGEU];
    const I32_IMMEDIATE: [ISA; 8] = [ISA::ADDI, ISA::SLLI, ISA::SLTI, ISA::SLTIU, ISA::XORI, ISA::SRLI, ISA::ORI, ISA::ANDI];
    const I_LOAD: [ISA; 8] = [ISA::LB, ISA::LH, ISA::LW, ISA::LD, ISA::LBU, ISA::LHU, ISA::LWU, ISA::UNKNOWN];
    const I_STORE: [ISA; 4] = [ISA::SB, ISA::SH, ISA::SW, ISA::SD];

    const I64_IMMEDIATE: [ISA; 8] = [ISA::ADDIW, ISA::SLLIW, ISA::UNKNOWN, ISA::UNKNOWN, ISA::UNKNOWN, ISA::SRLIW, ISA::UNKNOWN, ISA::UNKNOWN];
    const I64_ARITHMETIC: [ISA; 8] = [ISA::ADDW, ISA::SLLW, ISA::UNKNOWN, ISA::UNKNOWN, ISA::UNKNOWN, ISA::SRLW, ISA::UNKNOWN, ISA::UNKNOWN];

    fn get_immediate_32(opcode: u32) -> ISA {
        if opcode & 0xFC00_707F == 0x4000_5013 {
            ISA::SRAI
        } else {
            Self::I32_IMMEDIATE[opcode as usize >> 12 & 0b111]
        }
    }

    fn get_immediate_64(opcode: u32) -> ISA {
        if opcode & 0xFE00_707F == 0x4000_501B {
            ISA::SRAIW
        } else {
            Self::I64_IMMEDIATE[opcode as usize >> 12 & 0b111]
        }
    }

    fn get_arithmetic_32(opcode: u32) -> ISA {
        if opcode & 0xFE00_707F == 0x4000_0033 {
            ISA::SUB
        } else if opcode & 0xFE00_707F == 0x4000_5033 {
            ISA::SRA
        } else if opcode & 0xFE00_0000 != 0 {
            ISA::UNKNOWN
        } else {
            Self::I32_ARITHMETIC[opcode as usize >> 12 & 0b111]
        }
    }

    fn get_arithmetic_64(opcode: u32) -> ISA {
        if opcode & 0xFE00_707F == 0x4000_003B {
            ISA::SUBW
        } else if opcode & 0xFE00_707F == 0x4000_503B {
            ISA::SRAW
        } else if opcode & 0xFE00_0000 != 0 {
            ISA::UNKNOWN
        } else {
            Self::I64_ARITHMETIC[opcode as usize >> 12 & 0b111]
        }
    }

    pub fn from_opcode_32(opcode: u32) -> ISA {
        match opcode & 0b111_1111 {
            0b000_0011 => Self::I_LOAD[opcode as usize >> 12 & 0b111],
            0b000_1111 => ISA::FENCE,
            0b001_0011 => Self::get_immediate_32(opcode),
            0b001_0111 => ISA::AUIPC,
            0b001_1011 => Self::get_immediate_64(opcode),
            0b010_0011 => Self::I_STORE[opcode as usize >> 12 & 0b11],
            0b011_0011 => Self::get_arithmetic_32(opcode),
            0b011_0111 => ISA::LUI,
            0b011_1011 => Self::get_arithmetic_64(opcode),
            0b110_0011 => Self::I32_BRANCH[opcode as usize >> 12 & 0b111],
            0b110_0111 => ISA::JALR,
            0b110_1111 => ISA::JAL,
            0b111_0011 => if opcode == 0x0000_0073 { ISA::ECALL } else if opcode == 0x0010_0073 { ISA::EBREAK } else { ISA::UNKNOWN },
            _ => ISA::UNKNOWN,
        }
    }
}
