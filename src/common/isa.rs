use crate::common::{types::*, Instruction, Instruction32};

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
