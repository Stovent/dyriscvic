use crate::common::{extensions::*, instruction::*, types::*};
use crate::rvi::*;

#[derive(Clone, Copy, Debug)]
pub enum ISA {
    UNKNOWN,
    ADD,
    ADDI,
    AND,
    ANDI,
    AUIPC,
    BEQ,
    BGE,
    BGEU,
    BLT,
    BLTU,
    BNE,
    EBREAK,
    ECALL,
    FENCE,
    JAL,
    JALR,
    LB,
    LBU,
    LH,
    LHU,
    LUI,
    LW,
    OR,
    ORI,
    SB,
    SH,
    SLL,
    SLLI,
    SLT,
    SLTI,
    SLTIU,
    SLTU,
    SRA,
    SRAI,
    SRL,
    SRLI,
    SUB,
    SW,
    XOR,
    XORI,
}

const ISA_ARITHMETIC: [ISA; 8] = [ISA::ADD, ISA::SLL, ISA::SLT, ISA::SLTU, ISA::XOR, ISA::SRL, ISA::OR, ISA::AND];
const ISA_BRANCH: [ISA; 8] = [ISA::BEQ, ISA::BNE, ISA::UNKNOWN, ISA::UNKNOWN, ISA::BLT, ISA::BGE, ISA::BLTU, ISA::BGEU];
const ISA_IMMEDIATE: [ISA; 8] = [ISA::ADDI, ISA::SLLI, ISA::SLTI, ISA::SLTIU, ISA::XORI, ISA::SRLI, ISA::ORI, ISA::ANDI];
const ISA_LOAD: [ISA; 8] = [ISA::LB, ISA::LH, ISA::LW, ISA::UNKNOWN, ISA::LBU, ISA::LHU, ISA::UNKNOWN, ISA::UNKNOWN];
const ISA_STORE: [ISA; 4] = [ISA::SB, ISA::SH, ISA::SW, ISA::UNKNOWN];

fn get_instruction_from_opcode_immediate(opcode: u32) -> ISA {
    if opcode >> 12 & 0b111 == 0b101 && opcode >> 25 & 0b111_1111 == 0b010_0000 {
        ISA::SRAI
    } else {
        ISA_IMMEDIATE[opcode as usize >> 12 & 0b111]
    }
}

fn get_instruction_from_opcode_arithmetic(opcode: u32) -> ISA {
    if opcode >> 12 & 0b111 == 0 && opcode >> 25 & 0b111_1111 == 0b010_0000 {
        ISA::SUB
    } else if opcode >> 12 & 0b111 == 0b101 && opcode >> 25 & 0b111_1111 == 0b010_0000 {
        ISA::SRA
    } else if opcode >> 25 & 0b111_1111 != 0 {
        ISA::UNKNOWN
    } else {
        ISA_ARITHMETIC[opcode as usize >> 12 & 0b111]
    }
}

trait Format<U: Unsigned<S>, S: Signed<U>> {
    const FORMAT: [fn(ISA, U, u32) -> Instruction<U, S>; 41] = [
        Instruction::decode_type_fail, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_u, Instruction::decode_type_b, Instruction::decode_type_b,
        Instruction::decode_type_b,    Instruction::decode_type_b, Instruction::decode_type_b, Instruction::decode_type_b, Instruction::decode_type_empty, Instruction::decode_type_empty, Instruction::decode_type_i, Instruction::decode_type_j,
        Instruction::decode_type_i,    Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_u, Instruction::decode_type_i, Instruction::decode_type_r,
        Instruction::decode_type_i,    Instruction::decode_type_s, Instruction::decode_type_s, Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_i,
        Instruction::decode_type_r,    Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_s, Instruction::decode_type_r,
        Instruction::decode_type_i,
    ];
}

impl<U: Unsigned<S>, S: Signed<U>> Format<U, S> for ISA {}

pub trait GetISA<U: Unsigned<S>, S: Signed<U>> {
    fn get_instruction_from_opcode(pc: U, opcode: u32) -> Instruction<U, S>;
    fn get_isa(opcode: u32) -> ISA;
}

impl GetISA<u32, i32> for Instruction32 {
    fn get_instruction_from_opcode(pc: u32, opcode: u32) -> Instruction32 {
        let isa = Self::get_isa(opcode);
        ISA::FORMAT[isa as usize](isa, pc, opcode)
    }

    fn get_isa(opcode: u32) -> ISA {
        match opcode & 0b111_1111 {
            0b000_0011 => ISA_LOAD[opcode as usize >> 12 & 0b111],
            0b000_1111 => ISA::FENCE,
            0b001_0011 => get_instruction_from_opcode_immediate(opcode),
            0b001_0111 => ISA::AUIPC,
            0b010_0011 => ISA_STORE[opcode as usize >> 12 & 0b11],
            0b011_0011 => get_instruction_from_opcode_arithmetic(opcode),
            0b011_0111 => ISA::LUI,
            0b110_0011 => ISA_BRANCH[opcode as usize >> 12 & 0b111],
            0b110_0111 => ISA::JALR,
            0b110_1111 => ISA::JAL,
            0b111_0011 => if opcode == 0x0000_0073 { ISA::ECALL } else if opcode == 0x0010_0073 { ISA::EBREAK } else { ISA::UNKNOWN },
            _ => ISA::UNKNOWN,
        }
    }
}

impl GetISA<u64, i64> for Instruction64 {
    fn get_instruction_from_opcode(pc: u64, opcode: u32) -> Instruction64 {
        let isa = Self::get_isa(opcode);
        ISA::FORMAT[isa as usize](isa, pc, opcode)
    }

    fn get_isa(opcode: u32) -> ISA {
        match opcode & 0b111_1111 {
            0b000_0011 => ISA_LOAD[opcode as usize >> 12 & 0b111],
            0b000_1111 => ISA::FENCE,
            0b001_0011 => get_instruction_from_opcode_immediate(opcode),
            0b001_0111 => ISA::AUIPC,
            0b010_0011 => ISA_STORE[opcode as usize >> 12 & 0b11],
            0b011_0011 => get_instruction_from_opcode_arithmetic(opcode),
            0b011_0111 => ISA::LUI,
            0b110_0011 => ISA_BRANCH[opcode as usize >> 12 & 0b111],
            0b110_0111 => ISA::JALR,
            0b110_1111 => ISA::JAL,
            0b111_0011 => if opcode == 0x0000_0073 { ISA::ECALL } else if opcode == 0x0010_0073 { ISA::EBREAK } else { ISA::UNKNOWN },
            _ => ISA::UNKNOWN,
        }
    }
}

pub trait Execute<U: Unsigned<S>, S: Signed<U>, const N: usize> {
    const EXECUTE: [fn(&mut RVI<U, S, N>); 40] = [
        RVI::<U, S, N>::ADD,
        RVI::<U, S, N>::ADDI,
        RVI::<U, S, N>::AND,
        RVI::<U, S, N>::ANDI,
        RVI::<U, S, N>::AUIPC,
        RVI::<U, S, N>::BEQ,
        RVI::<U, S, N>::BGE,
        RVI::<U, S, N>::BGEU,
        RVI::<U, S, N>::BLT,
        RVI::<U, S, N>::BLTU,
        RVI::<U, S, N>::BNE,
        RVI::<U, S, N>::EBREAK,
        RVI::<U, S, N>::ECALL,
        RVI::<U, S, N>::FENCE,
        RVI::<U, S, N>::JAL,
        RVI::<U, S, N>::JALR,
        RVI::<U, S, N>::LB,
        RVI::<U, S, N>::LBU,
        RVI::<U, S, N>::LH,
        RVI::<U, S, N>::LHU,
        RVI::<U, S, N>::LUI,
        RVI::<U, S, N>::LW,
        RVI::<U, S, N>::OR,
        RVI::<U, S, N>::ORI,
        RVI::<U, S, N>::SB,
        RVI::<U, S, N>::SH,
        RVI::<U, S, N>::SLL,
        RVI::<U, S, N>::SLLI,
        RVI::<U, S, N>::SLT,
        RVI::<U, S, N>::SLTI,
        RVI::<U, S, N>::SLTIU,
        RVI::<U, S, N>::SLTU,
        RVI::<U, S, N>::SRA,
        RVI::<U, S, N>::SRAI,
        RVI::<U, S, N>::SRL,
        RVI::<U, S, N>::SRLI,
        RVI::<U, S, N>::SUB,
        RVI::<U, S, N>::SW,
        RVI::<U, S, N>::XOR,
        RVI::<U, S, N>::XORI,
    ];
}

pub trait Disassemble<U: Unsigned<S>, S: Signed<U>, const N: usize> {
    const DISASSEMBLE: [fn(Instruction<U, S>); 40] = [
        RVI::<U, S, N>::disassemble_ADD,
        RVI::<U, S, N>::disassemble_ADDI,
        RVI::<U, S, N>::disassemble_AND,
        RVI::<U, S, N>::disassemble_ANDI,
        RVI::<U, S, N>::disassemble_AUIPC,
        RVI::<U, S, N>::disassemble_BEQ,
        RVI::<U, S, N>::disassemble_BGE,
        RVI::<U, S, N>::disassemble_BGEU,
        RVI::<U, S, N>::disassemble_BLT,
        RVI::<U, S, N>::disassemble_BLTU,
        RVI::<U, S, N>::disassemble_BNE,
        RVI::<U, S, N>::disassemble_EBREAK,
        RVI::<U, S, N>::disassemble_ECALL,
        RVI::<U, S, N>::disassemble_FENCE,
        RVI::<U, S, N>::disassemble_JAL,
        RVI::<U, S, N>::disassemble_JALR,
        RVI::<U, S, N>::disassemble_LB,
        RVI::<U, S, N>::disassemble_LBU,
        RVI::<U, S, N>::disassemble_LH,
        RVI::<U, S, N>::disassemble_LHU,
        RVI::<U, S, N>::disassemble_LUI,
        RVI::<U, S, N>::disassemble_LW,
        RVI::<U, S, N>::disassemble_OR,
        RVI::<U, S, N>::disassemble_ORI,
        RVI::<U, S, N>::disassemble_SB,
        RVI::<U, S, N>::disassemble_SH,
        RVI::<U, S, N>::disassemble_SLL,
        RVI::<U, S, N>::disassemble_SLLI,
        RVI::<U, S, N>::disassemble_SLT,
        RVI::<U, S, N>::disassemble_SLTI,
        RVI::<U, S, N>::disassemble_SLTIU,
        RVI::<U, S, N>::disassemble_SLTU,
        RVI::<U, S, N>::disassemble_SRA,
        RVI::<U, S, N>::disassemble_SRAI,
        RVI::<U, S, N>::disassemble_SRL,
        RVI::<U, S, N>::disassemble_SRLI,
        RVI::<U, S, N>::disassemble_SUB,
        RVI::<U, S, N>::disassemble_SW,
        RVI::<U, S, N>::disassemble_XOR,
        RVI::<U, S, N>::disassemble_XORI,
    ];
}
