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

trait Format<PC: Unsigned, IMM: Signed> {
    const FORMAT: [fn(ISA, PC, u32) -> Instruction<PC, IMM>; 41] = [
        Instruction::decode_type_fail, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_u, Instruction::decode_type_b, Instruction::decode_type_b,
        Instruction::decode_type_b,    Instruction::decode_type_b, Instruction::decode_type_b, Instruction::decode_type_b, Instruction::decode_type_empty, Instruction::decode_type_empty, Instruction::decode_type_i, Instruction::decode_type_j,
        Instruction::decode_type_i,    Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_i, Instruction::decode_type_u, Instruction::decode_type_i, Instruction::decode_type_r,
        Instruction::decode_type_i,    Instruction::decode_type_s, Instruction::decode_type_s, Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_i, Instruction::decode_type_i,
        Instruction::decode_type_r,    Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_r, Instruction::decode_type_s, Instruction::decode_type_r,
        Instruction::decode_type_i,
    ];
}

impl<PC: Unsigned, IMM: Signed> Format<PC, IMM> for ISA {}

pub trait GetISA<PC: Unsigned> {
    fn get_instruction_from_opcode(pc: PC, opcode: u32) -> Self;
    fn get_isa(opcode: u32) -> ISA;
}

impl GetISA<u32> for Instruction32 {
    fn get_instruction_from_opcode(pc: u32, opcode: u32) -> Instruction32 {
        let isa = Instruction32::get_isa(opcode);
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

impl GetISA<u64> for Instruction64 {
    fn get_instruction_from_opcode(pc: u64, opcode: u32) -> Instruction64 {
        let isa = Instruction64::get_isa(opcode);
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

pub trait Execute<'a, const N: usize> {
    const EXECUTE: [fn(&mut RV32<'a, N>); 41] = [
        RV32::UNKNOWN,
        RV32::ADD,
        RV32::ADDI,
        RV32::AND,
        RV32::ANDI,
        RV32::AUIPC,
        RV32::BEQ,
        RV32::BGE,
        RV32::BGEU,
        RV32::BLT,
        RV32::BLTU,
        RV32::BNE,
        RV32::EBREAK,
        RV32::ECALL,
        RV32::FENCE,
        RV32::JAL,
        RV32::JALR,
        RV32::LB,
        RV32::LBU,
        RV32::LH,
        RV32::LHU,
        RV32::LUI,
        RV32::LW,
        RV32::OR,
        RV32::ORI,
        RV32::SB,
        RV32::SH,
        RV32::SLL,
        RV32::SLLI,
        RV32::SLT,
        RV32::SLTI,
        RV32::SLTIU,
        RV32::SLTU,
        RV32::SRA,
        RV32::SRAI,
        RV32::SRL,
        RV32::SRLI,
        RV32::SUB,
        RV32::SW,
        RV32::XOR,
        RV32::XORI,
    ];
}

pub trait Disassemble<'a, PC: Unsigned, IMM: Signed, const N: usize> {
    const DISASSEMBLE: [fn(Instruction<PC, IMM>); 41] = [
        RV32::<N>::disassemble_UNKNOWN,
        RV32::<N>::disassemble_ADD,
        RV32::<N>::disassemble_ADDI,
        RV32::<N>::disassemble_AND,
        RV32::<N>::disassemble_ANDI,
        RV32::<N>::disassemble_AUIPC,
        RV32::<N>::disassemble_BEQ,
        RV32::<N>::disassemble_BGE,
        RV32::<N>::disassemble_BGEU,
        RV32::<N>::disassemble_BLT,
        RV32::<N>::disassemble_BLTU,
        RV32::<N>::disassemble_BNE,
        RV32::<N>::disassemble_EBREAK,
        RV32::<N>::disassemble_ECALL,
        RV32::<N>::disassemble_FENCE,
        RV32::<N>::disassemble_JAL,
        RV32::<N>::disassemble_JALR,
        RV32::<N>::disassemble_LB,
        RV32::<N>::disassemble_LBU,
        RV32::<N>::disassemble_LH,
        RV32::<N>::disassemble_LHU,
        RV32::<N>::disassemble_LUI,
        RV32::<N>::disassemble_LW,
        RV32::<N>::disassemble_OR,
        RV32::<N>::disassemble_ORI,
        RV32::<N>::disassemble_SB,
        RV32::<N>::disassemble_SH,
        RV32::<N>::disassemble_SLL,
        RV32::<N>::disassemble_SLLI,
        RV32::<N>::disassemble_SLT,
        RV32::<N>::disassemble_SLTI,
        RV32::<N>::disassemble_SLTIU,
        RV32::<N>::disassemble_SLTU,
        RV32::<N>::disassemble_SRA,
        RV32::<N>::disassemble_SRAI,
        RV32::<N>::disassemble_SRL,
        RV32::<N>::disassemble_SRLI,
        RV32::<N>::disassemble_SUB,
        RV32::<N>::disassemble_SW,
        RV32::<N>::disassemble_XOR,
        RV32::<N>::disassemble_XORI,
    ];
}
