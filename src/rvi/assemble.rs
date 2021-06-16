//! Instruction assembler. Parameters are in the same order as in the mnemonic.

use crate::common::instruction::*;

// I32
pub fn ADD(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 0, rs1, rs2, 0)
}

pub fn ADDI(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 0, rs1, imm)
}

pub fn AND(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 7, rs1, rs2, 0)
}

pub fn ANDI(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 7, rs1, imm)
}

pub fn AUIPC(rd: u8, imm: u32) -> u32 {
    encode_type_u(0b0010111, rd, imm)
}

pub fn BEQ(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 0, rs1, rs2, imm)
}

pub fn BGE(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 5, rs1, rs2, imm)
}

pub fn BGEU(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 7, rs1, rs2, imm)
}

pub fn BLT(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 4, rs1, rs2, imm)
}

pub fn BLTU(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 6, rs1, rs2, imm)
}

pub fn BNE(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 1, rs1, rs2, imm)
}

pub fn EBREAK() -> u32 {
    0b000000000001_00000_000_00000_1110011
}

pub fn ECALL() -> u32 {
    0b000000000000_00000_000_00000_1110011
}

pub fn FENCE(rd: u8, rs1: u8, succ: u8, pred: u8, fm: u8) -> u32 {
    let imm = (fm as u32) << 8 & 0xF00 | (pred as u32) << 4 & 0xF0 | (succ as u32) & 0xF;
    encode_type_i(0b0001111, rd, 0, rs1, imm)
}

pub fn JAL(rd: u8, imm: u32) -> u32 {
    encode_type_j(0b1101111, rd, imm)
}

pub fn JALR(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b1100111, rd, 0, rs1, imm)
}

pub fn LB(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 0, rs1, imm)
}

pub fn LBU(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 4, rs1, imm)
}

pub fn LH(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 1, rs1, imm)
}

pub fn LHU(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 5, rs1, imm)
}

pub fn LUI(rd: u8, imm: u32) -> u32 {
    encode_type_u(0b0110111, rd, imm)
}

pub fn LW(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 2, rs1, imm)
}

pub fn OR(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 6, rs1, rs2, 0)
}

pub fn ORI(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 6, rs1, imm)
}

pub fn SB(rs2: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_s(0b0100011, 0, rs1, rs2, imm)
}

pub fn SH(rs2: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_s(0b0100011, 1, rs1, rs2, imm)
}

pub fn SLL(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 1, rs1, rs2, 0)
}

pub fn SLLI(rd: u8, rs1: u8, shamt: u8) -> u32 {
    encode_type_r(0b0010011, rd, 1, rs1, shamt, 0)
}

pub fn SLT(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 2, rs1, rs2, 0)
}

pub fn SLTI(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 2, rs1, imm)
}

pub fn SLTIU(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 3, rs1, imm)
}

pub fn SLTU(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 3, rs1, rs2, 0)
}

pub fn SRA(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 5, rs1, rs2, 0b0100000)
}

pub fn SRAI(rd: u8, rs1: u8, shamt: u8) -> u32 {
    encode_type_r(0b0010011, rd, 5, rs1, shamt, 0b0100000)
}

pub fn SRL(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 5, rs1, rs2, 0)
}

pub fn SRLI(rd: u8, rs1: u8, shamt: u8) -> u32 {
    encode_type_r(0b0010011, rd, 5, rs1, shamt, 0)
}

pub fn SUB(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 0, rs1, rs2, 0b0100000)
}

pub fn SW(rs2: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_s(0b0100011, 2, rs1, rs2, imm)
}

pub fn XOR(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 4, rs1, rs2, 0)
}

pub fn XORI(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 4, rs1, imm)
}

// I64
pub fn ADDIW(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0011011, rd, 0, rs1, imm)
}

pub fn ADDW(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0111011, rd, 0, rs1, rs2, 0)
}

pub fn LD(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 3, rs1, imm)
}

pub fn LWU(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 6, rs1, imm)
}

pub fn SD(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_s(0b0100011, 3, rs1, rs2, imm)
}

pub fn SLLIW(rd: u8, rs1: u8, shamt: u8) -> u32 {
    encode_type_r(0b0011011, rd, 1, rs1, shamt, 0)
}

pub fn SLLW(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0111011, rd, 1, rs1, rs2, 0)
}

pub fn SRAIW(rd: u8, rs1: u8, shamt: u8) -> u32 {
    encode_type_r(0b0011011, rd, 5, rs1, shamt, 0b0100000)
}

pub fn SRAW(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0111011, rd, 5, rs1, rs2, 0b0100000)
}

pub fn SRLIW(rd: u8, rs1: u8, shamt: u8) -> u32 {
    encode_type_r(0b0011011, rd, 5, rs1, shamt, 0)
}

pub fn SRLW(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0111011, rd, 5, rs1, rs2, 0)
}

pub fn SUBW(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0111011, rd, 0, rs1, rs2, 0b0100000)
}
