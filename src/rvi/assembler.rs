//! Instruction assembler. Parameters are in the same order as in the mnemonic.

use crate::common::instruction::*;

// I32
pub const fn ADD(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 0, rs1, rs2, 0)
}

pub const fn ADDI(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 0, rs1, imm)
}

pub const fn AND(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 7, rs1, rs2, 0)
}

pub const fn ANDI(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 7, rs1, imm)
}

pub const fn AUIPC(rd: u8, imm: u32) -> u32 {
    encode_type_u(0b0010111, rd, imm)
}

pub const fn BEQ(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 0, rs1, rs2, imm)
}

pub const fn BGE(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 5, rs1, rs2, imm)
}

pub const fn BGEU(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 7, rs1, rs2, imm)
}

pub const fn BLT(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 4, rs1, rs2, imm)
}

pub const fn BLTU(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 6, rs1, rs2, imm)
}

pub const fn BNE(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_b(0b1100011, 1, rs1, rs2, imm)
}

pub const fn EBREAK() -> u32 {
    0b000000000001_00000_000_00000_1110011
}

pub const fn ECALL() -> u32 {
    0b000000000000_00000_000_00000_1110011
}

pub const fn FENCE(rd: u8, rs1: u8, succ: u8, pred: u8, fm: u8) -> u32 {
    let imm = (fm as u32) << 8 & 0xF00 | (pred as u32) << 4 & 0xF0 | (succ as u32) & 0xF;
    encode_type_i(0b0001111, rd, 0, rs1, imm)
}

pub const fn JAL(rd: u8, imm: u32) -> u32 {
    encode_type_j(0b1101111, rd, imm)
}

pub const fn JALR(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b1100111, rd, 0, rs1, imm)
}

pub const fn LB(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 0, rs1, imm)
}

pub const fn LBU(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 4, rs1, imm)
}

pub const fn LH(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 1, rs1, imm)
}

pub const fn LHU(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 5, rs1, imm)
}

pub const fn LUI(rd: u8, imm: u32) -> u32 {
    encode_type_u(0b0110111, rd, imm)
}

pub const fn LW(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 2, rs1, imm)
}

pub const fn OR(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 6, rs1, rs2, 0)
}

pub const fn ORI(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 6, rs1, imm)
}

pub const fn SB(rs2: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_s(0b0100011, 0, rs1, rs2, imm)
}

pub const fn SH(rs2: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_s(0b0100011, 1, rs1, rs2, imm)
}

pub const fn SLL(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 1, rs1, rs2, 0)
}

pub const fn SLLI(rd: u8, rs1: u8, shamt: u32) -> u32 {
    encode_type_i(0b0010011, rd, 1, rs1, shamt & 0x3F)
}

pub const fn SLT(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 2, rs1, rs2, 0)
}

pub const fn SLTI(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 2, rs1, imm)
}

pub const fn SLTIU(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 3, rs1, imm)
}

pub const fn SLTU(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 3, rs1, rs2, 0)
}

pub const fn SRA(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 5, rs1, rs2, 0b0100000)
}

pub const fn SRAI(rd: u8, rs1: u8, shamt: u32) -> u32 {
    encode_type_i(0b0010011, rd, 5, rs1, 0x400u32 | (shamt & 0x3F))
}

pub const fn SRL(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 5, rs1, rs2, 0)
}

pub const fn SRLI(rd: u8, rs1: u8, shamt: u32) -> u32 {
    encode_type_i(0b0010011, rd, 5, rs1, shamt & 0x3F)
}

pub const fn SUB(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 0, rs1, rs2, 0b0100000)
}

pub const fn SW(rs2: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_s(0b0100011, 2, rs1, rs2, imm)
}

pub const fn XOR(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0110011, rd, 4, rs1, rs2, 0)
}

pub const fn XORI(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0010011, rd, 4, rs1, imm)
}

// I64
pub const fn ADDIW(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0011011, rd, 0, rs1, imm)
}

pub const fn ADDW(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0111011, rd, 0, rs1, rs2, 0)
}

pub const fn LD(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 3, rs1, imm)
}

pub const fn LWU(rd: u8, rs1: u8, imm: u32) -> u32 {
    encode_type_i(0b0000011, rd, 6, rs1, imm)
}

pub const fn SD(rs1: u8, rs2: u8, imm: u32) -> u32 {
    encode_type_s(0b0100011, 3, rs1, rs2, imm)
}

pub const fn SLLIW(rd: u8, rs1: u8, shamt: u8) -> u32 {
    encode_type_r(0b0011011, rd, 1, rs1, shamt, 0)
}

pub const fn SLLW(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0111011, rd, 1, rs1, rs2, 0)
}

pub const fn SRAIW(rd: u8, rs1: u8, shamt: u8) -> u32 {
    encode_type_r(0b0011011, rd, 5, rs1, shamt, 0b0100000)
}

pub const fn SRAW(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0111011, rd, 5, rs1, rs2, 0b0100000)
}

pub const fn SRLIW(rd: u8, rs1: u8, shamt: u8) -> u32 {
    encode_type_r(0b0011011, rd, 5, rs1, shamt, 0)
}

pub const fn SRLW(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0111011, rd, 5, rs1, rs2, 0)
}

pub const fn SUBW(rd: u8, rs1: u8, rs2: u8) -> u32 {
    encode_type_r(0b0111011, rd, 0, rs1, rs2, 0b0100000)
}
