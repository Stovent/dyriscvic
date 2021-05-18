use crate::common::{instruction::*, types::*};
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

impl<PC: Unsigned, IMM: Signed> Format<PC, IMM> for ISA {}

pub fn get_instruction32_from_opcode(pc: u32, opcode: u32) -> Instruction32 {
    let isa = get_isa(opcode);
    ISA::FORMAT[isa as usize](isa, pc, opcode)
}

const ISA_ARITHMETIC: [ISA; 8] = [ISA::ADD, ISA::SLL, ISA::SLT, ISA::SLTU, ISA::XOR, ISA::SRL, ISA::OR, ISA::AND];
const ISA_BRANCH: [ISA; 8] = [ISA::BEQ, ISA::BNE, ISA::UNKNOWN, ISA::UNKNOWN, ISA::BLT, ISA::BGE, ISA::BLTU, ISA::BGEU];
const ISA_IMMEDIATE: [ISA; 8] = [ISA::ADDI, ISA::SLLI, ISA::SLTI, ISA::SLTIU, ISA::XORI, ISA::SRLI, ISA::ORI, ISA::ANDI];
const ISA_LOAD: [ISA; 8] = [ISA::LB, ISA::LH, ISA::LW, ISA::UNKNOWN, ISA::LBU, ISA::LHU, ISA::UNKNOWN, ISA::UNKNOWN];
const ISA_STORE: [ISA; 4] = [ISA::SB, ISA::SH, ISA::SW, ISA::UNKNOWN];

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

pub trait I<'a, const N: usize> {
    const EXECUTE: [fn(&mut RV32<'a, N>); 41] = [
        RV32::UNKNOWN, RV32::ADD, RV32::ADDI, RV32::AND,  RV32::ANDI,   RV32::AUIPC, RV32::BEQ,   RV32::BGE,
        RV32::BGEU,    RV32::BLT, RV32::BLTU, RV32::BNE,  RV32::EBREAK, RV32::ECALL, RV32::FENCE, RV32::JAL,
        RV32::JALR,    RV32::LB,  RV32::LBU,  RV32::LH,   RV32::LHU,    RV32::LUI,   RV32::LW,    RV32::OR,
        RV32::ORI,     RV32::SB,  RV32::SH,   RV32::SLL,  RV32::SLLI,   RV32::SLT,   RV32::SLTI,  RV32::SLTIU,
        RV32::SLTU,    RV32::SRA, RV32::SRAI, RV32::SRL,  RV32::SRLI,   RV32::SUB,   RV32::SW,    RV32::XOR,
        RV32::XORI,
    ];

    fn UNKNOWN(&mut self);
    fn ADD(&mut self);
    fn ADDI(&mut self);
    fn AND(&mut self);
    fn ANDI(&mut self);
    fn AUIPC(&mut self);
    fn BEQ(&mut self);
    fn BGE(&mut self);
    fn BGEU(&mut self);
    fn BLT(&mut self);
    fn BLTU(&mut self);
    fn BNE(&mut self);
    fn EBREAK(&mut self);
    fn ECALL(&mut self);
    fn FENCE(&mut self);
    fn JAL(&mut self);
    fn JALR(&mut self);
    fn LB(&mut self);
    fn LBU(&mut self);
    fn LH(&mut self);
    fn LHU(&mut self);
    fn LUI(&mut self);
    fn LW(&mut self);
    fn OR(&mut self);
    fn ORI(&mut self);
    fn SB(&mut self);
    fn SH(&mut self);
    fn SLL(&mut self);
    fn SLLI(&mut self);
    fn SLT(&mut self);
    fn SLTI(&mut self);
    fn SLTIU(&mut self);
    fn SLTU(&mut self);
    fn SRA(&mut self);
    fn SRAI(&mut self);
    fn SRL(&mut self);
    fn SRLI(&mut self);
    fn SUB(&mut self);
    fn SW(&mut self);
    fn XOR(&mut self);
    fn XORI(&mut self);
}

pub trait DisassembleI<'a, PC: Unsigned, IMM: Signed, const N: usize> {
    const DISASSEMBLE: [fn(Instruction<PC, IMM>); 41] = [
        RV32::<N>::disassemble_UNKNOWN, RV32::<N>::disassemble_ADD, RV32::<N>::disassemble_ADDI, RV32::<N>::disassemble_AND,  RV32::<N>::disassemble_ANDI,   RV32::<N>::disassemble_AUIPC, RV32::<N>::disassemble_BEQ,   RV32::<N>::disassemble_BGE,
        RV32::<N>::disassemble_BGEU,    RV32::<N>::disassemble_BLT, RV32::<N>::disassemble_BLTU, RV32::<N>::disassemble_BNE,  RV32::<N>::disassemble_EBREAK, RV32::<N>::disassemble_ECALL, RV32::<N>::disassemble_FENCE, RV32::<N>::disassemble_JAL,
        RV32::<N>::disassemble_JALR,    RV32::<N>::disassemble_LB,  RV32::<N>::disassemble_LBU,  RV32::<N>::disassemble_LH,   RV32::<N>::disassemble_LHU,    RV32::<N>::disassemble_LUI,   RV32::<N>::disassemble_LW,    RV32::<N>::disassemble_OR,
        RV32::<N>::disassemble_ORI,     RV32::<N>::disassemble_SB,  RV32::<N>::disassemble_SH,   RV32::<N>::disassemble_SLL,  RV32::<N>::disassemble_SLLI,   RV32::<N>::disassemble_SLT,   RV32::<N>::disassemble_SLTI,  RV32::<N>::disassemble_SLTIU,
        RV32::<N>::disassemble_SLTU,    RV32::<N>::disassemble_SRA, RV32::<N>::disassemble_SRAI, RV32::<N>::disassemble_SRL,  RV32::<N>::disassemble_SRLI,   RV32::<N>::disassemble_SUB,   RV32::<N>::disassemble_SW,    RV32::<N>::disassemble_XOR,
        RV32::<N>::disassemble_XORI,
    ];

    fn disassemble_UNKNOWN(inst: Instruction<PC, IMM>);
    fn disassemble_ADD(inst: Instruction<PC, IMM>);
    fn disassemble_ADDI(inst: Instruction<PC, IMM>);
    fn disassemble_AND(inst: Instruction<PC, IMM>);
    fn disassemble_ANDI(inst: Instruction<PC, IMM>);
    fn disassemble_AUIPC(inst: Instruction<PC, IMM>);
    fn disassemble_BEQ(inst: Instruction<PC, IMM>);
    fn disassemble_BGE(inst: Instruction<PC, IMM>);
    fn disassemble_BGEU(inst: Instruction<PC, IMM>);
    fn disassemble_BLT(inst: Instruction<PC, IMM>);
    fn disassemble_BLTU(inst: Instruction<PC, IMM>);
    fn disassemble_BNE(inst: Instruction<PC, IMM>);
    fn disassemble_EBREAK(inst: Instruction<PC, IMM>);
    fn disassemble_ECALL(inst: Instruction<PC, IMM>);
    fn disassemble_FENCE(inst: Instruction<PC, IMM>);
    fn disassemble_JAL(inst: Instruction<PC, IMM>);
    fn disassemble_JALR(inst: Instruction<PC, IMM>);
    fn disassemble_LB(inst: Instruction<PC, IMM>);
    fn disassemble_LBU(inst: Instruction<PC, IMM>);
    fn disassemble_LH(inst: Instruction<PC, IMM>);
    fn disassemble_LHU(inst: Instruction<PC, IMM>);
    fn disassemble_LUI(inst: Instruction<PC, IMM>);
    fn disassemble_LW(inst: Instruction<PC, IMM>);
    fn disassemble_OR(inst: Instruction<PC, IMM>);
    fn disassemble_ORI(inst: Instruction<PC, IMM>);
    fn disassemble_SB(inst: Instruction<PC, IMM>);
    fn disassemble_SH(inst: Instruction<PC, IMM>);
    fn disassemble_SLL(inst: Instruction<PC, IMM>);
    fn disassemble_SLLI(inst: Instruction<PC, IMM>);
    fn disassemble_SLT(inst: Instruction<PC, IMM>);
    fn disassemble_SLTI(inst: Instruction<PC, IMM>);
    fn disassemble_SLTIU(inst: Instruction<PC, IMM>);
    fn disassemble_SLTU(inst: Instruction<PC, IMM>);
    fn disassemble_SRA(inst: Instruction<PC, IMM>);
    fn disassemble_SRAI(inst: Instruction<PC, IMM>);
    fn disassemble_SRL(inst: Instruction<PC, IMM>);
    fn disassemble_SRLI(inst: Instruction<PC, IMM>);
    fn disassemble_SUB(inst: Instruction<PC, IMM>);
    fn disassemble_SW(inst: Instruction<PC, IMM>);
    fn disassemble_XOR(inst: Instruction<PC, IMM>);
    fn disassemble_XORI(inst: Instruction<PC, IMM>);
}
