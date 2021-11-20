//! Instruction set stuff.

use crate::common::instruction::*;
use crate::public::*;
use crate::rvi::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isa {
    UNKNOWN,

    ADD, // I32
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

    ADDIW, // I64
    ADDW,
    LD,
    LWU,
    SD,
    SLLIW,
    SLLW,
    SRAIW,
    SRAW,
    SRLIW,
    SRLW,
    SUBW,

    _SIZE, // used internally by dyriscvic, not a real instruction
}

/// This struct is meant to built a constexpr look-up table for all the ISA.
#[derive(Clone, Copy)]
pub struct IsaEntry<EEI: ExecutionEnvironmentInterface> {
    /// The ISA itself.
    pub isa: Isa,
    /// The decoding instruction.
    pub decoder: fn(Isa, u64, u32) -> Instruction,
    /// The execute method.
    pub execute: fn(&mut RV64I<EEI>),
    /// The disassemble method.
    pub disassemble: fn(Instruction, bool) -> String,
}

impl<EEI: ExecutionEnvironmentInterface> RV64I<EEI> {
    /// Maps an Isa to its associated decoding, executing and disassembling functions.
    pub const ISA_LUT: [IsaEntry<EEI>; Isa::_SIZE as usize] = [
        IsaEntry { isa: Isa::UNKNOWN, decoder: Instruction::empty,         execute: Self::UNKNOWN, disassemble: Self::disassemble_UNKNOWN },
        IsaEntry { isa: Isa::ADD,     decoder: Instruction::decode_type_r, execute: Self::ADD,     disassemble: Self::disassemble_ADD },
        IsaEntry { isa: Isa::ADDI,    decoder: Instruction::decode_type_i, execute: Self::ADDI,    disassemble: Self::disassemble_ADDI },
        IsaEntry { isa: Isa::AND,     decoder: Instruction::decode_type_r, execute: Self::AND,     disassemble: Self::disassemble_AND },
        IsaEntry { isa: Isa::ANDI,    decoder: Instruction::decode_type_i, execute: Self::ANDI,    disassemble: Self::disassemble_ANDI },
        IsaEntry { isa: Isa::AUIPC,   decoder: Instruction::decode_type_u, execute: Self::AUIPC,   disassemble: Self::disassemble_AUIPC },
        IsaEntry { isa: Isa::BEQ,     decoder: Instruction::decode_type_b, execute: Self::BEQ,     disassemble: Self::disassemble_BEQ },
        IsaEntry { isa: Isa::BGE,     decoder: Instruction::decode_type_b, execute: Self::BGE,     disassemble: Self::disassemble_BGE },
        IsaEntry { isa: Isa::BGEU,    decoder: Instruction::decode_type_b, execute: Self::BGEU,    disassemble: Self::disassemble_BGEU },
        IsaEntry { isa: Isa::BLT,     decoder: Instruction::decode_type_b, execute: Self::BLT,     disassemble: Self::disassemble_BLT },
        IsaEntry { isa: Isa::BLTU,    decoder: Instruction::decode_type_b, execute: Self::BLTU,    disassemble: Self::disassemble_BLTU },
        IsaEntry { isa: Isa::BNE,     decoder: Instruction::decode_type_b, execute: Self::BNE,     disassemble: Self::disassemble_BNE },
        IsaEntry { isa: Isa::EBREAK,  decoder: Instruction::empty,         execute: Self::EBREAK,  disassemble: Self::disassemble_EBREAK },
        IsaEntry { isa: Isa::ECALL,   decoder: Instruction::empty,         execute: Self::ECALL,   disassemble: Self::disassemble_ECALL },
        IsaEntry { isa: Isa::FENCE,   decoder: Instruction::decode_type_i, execute: Self::FENCE,   disassemble: Self::disassemble_FENCE },
        IsaEntry { isa: Isa::JAL,     decoder: Instruction::decode_type_j, execute: Self::JAL,     disassemble: Self::disassemble_JAL },
        IsaEntry { isa: Isa::JALR,    decoder: Instruction::decode_type_i, execute: Self::JALR,    disassemble: Self::disassemble_JALR },
        IsaEntry { isa: Isa::LB,      decoder: Instruction::decode_type_i, execute: Self::LB,      disassemble: Self::disassemble_LB },
        IsaEntry { isa: Isa::LBU,     decoder: Instruction::decode_type_i, execute: Self::LBU,     disassemble: Self::disassemble_LBU },
        IsaEntry { isa: Isa::LH,      decoder: Instruction::decode_type_i, execute: Self::LH,      disassemble: Self::disassemble_LH },
        IsaEntry { isa: Isa::LHU,     decoder: Instruction::decode_type_i, execute: Self::LHU,     disassemble: Self::disassemble_LHU },
        IsaEntry { isa: Isa::LUI,     decoder: Instruction::decode_type_u, execute: Self::LUI,     disassemble: Self::disassemble_LUI },
        IsaEntry { isa: Isa::LW,      decoder: Instruction::decode_type_i, execute: Self::LW,      disassemble: Self::disassemble_LW },
        IsaEntry { isa: Isa::OR,      decoder: Instruction::decode_type_r, execute: Self::OR,      disassemble: Self::disassemble_OR },
        IsaEntry { isa: Isa::ORI,     decoder: Instruction::decode_type_i, execute: Self::ORI,     disassemble: Self::disassemble_ORI },
        IsaEntry { isa: Isa::SB,      decoder: Instruction::decode_type_s, execute: Self::SB,      disassemble: Self::disassemble_SB },
        IsaEntry { isa: Isa::SH,      decoder: Instruction::decode_type_s, execute: Self::SH,      disassemble: Self::disassemble_SH },
        IsaEntry { isa: Isa::SLL,     decoder: Instruction::decode_type_r, execute: Self::SLL,     disassemble: Self::disassemble_SLL },
        IsaEntry { isa: Isa::SLLI,    decoder: Instruction::decode_type_i, execute: Self::SLLI,    disassemble: Self::disassemble_SLLI },
        IsaEntry { isa: Isa::SLT,     decoder: Instruction::decode_type_r, execute: Self::SLT,     disassemble: Self::disassemble_SLT },
        IsaEntry { isa: Isa::SLTI,    decoder: Instruction::decode_type_i, execute: Self::SLTI,    disassemble: Self::disassemble_SLTI },
        IsaEntry { isa: Isa::SLTIU,   decoder: Instruction::decode_type_i, execute: Self::SLTIU,   disassemble: Self::disassemble_SLTIU },
        IsaEntry { isa: Isa::SLTU,    decoder: Instruction::decode_type_r, execute: Self::SLTU,    disassemble: Self::disassemble_SLTU },
        IsaEntry { isa: Isa::SRA,     decoder: Instruction::decode_type_r, execute: Self::SRA,     disassemble: Self::disassemble_SRA },
        IsaEntry { isa: Isa::SRAI,    decoder: Instruction::decode_type_i, execute: Self::SRAI,    disassemble: Self::disassemble_SRAI },
        IsaEntry { isa: Isa::SRL,     decoder: Instruction::decode_type_r, execute: Self::SRL,     disassemble: Self::disassemble_SRL },
        IsaEntry { isa: Isa::SRLI,    decoder: Instruction::decode_type_i, execute: Self::SRLI,    disassemble: Self::disassemble_SRLI },
        IsaEntry { isa: Isa::SUB,     decoder: Instruction::decode_type_r, execute: Self::SUB,     disassemble: Self::disassemble_SUB },
        IsaEntry { isa: Isa::SW,      decoder: Instruction::decode_type_s, execute: Self::SW,      disassemble: Self::disassemble_SW },
        IsaEntry { isa: Isa::XOR,     decoder: Instruction::decode_type_r, execute: Self::XOR,     disassemble: Self::disassemble_XOR },
        IsaEntry { isa: Isa::XORI,    decoder: Instruction::decode_type_i, execute: Self::XORI,    disassemble: Self::disassemble_XORI },

        IsaEntry { isa: Isa::ADDIW,   decoder: Instruction::decode_type_i, execute: Self::ADDIW,   disassemble: Self::disassemble_ADDIW },
        IsaEntry { isa: Isa::ADDW,    decoder: Instruction::decode_type_r, execute: Self::ADDW,    disassemble: Self::disassemble_ADDW },
        IsaEntry { isa: Isa::LD,      decoder: Instruction::decode_type_i, execute: Self::LD,      disassemble: Self::disassemble_LD },
        IsaEntry { isa: Isa::LWU,     decoder: Instruction::decode_type_i, execute: Self::LWU,     disassemble: Self::disassemble_LWU },
        IsaEntry { isa: Isa::SD,      decoder: Instruction::decode_type_s, execute: Self::SD,      disassemble: Self::disassemble_SD },
        IsaEntry { isa: Isa::SLLIW,   decoder: Instruction::decode_type_r, execute: Self::SLLIW,   disassemble: Self::disassemble_SLLIW }, // type i or type r ?
        IsaEntry { isa: Isa::SLLW,    decoder: Instruction::decode_type_r, execute: Self::SLLW,    disassemble: Self::disassemble_SLLW },
        IsaEntry { isa: Isa::SRAIW,   decoder: Instruction::decode_type_r, execute: Self::SRAIW,   disassemble: Self::disassemble_SRAIW }, // type i or type r ?
        IsaEntry { isa: Isa::SRAW,    decoder: Instruction::decode_type_r, execute: Self::SRAW,    disassemble: Self::disassemble_SRAW },
        IsaEntry { isa: Isa::SRLIW,   decoder: Instruction::decode_type_r, execute: Self::SRLIW,   disassemble: Self::disassemble_SRLIW }, // type i or type r ?
        IsaEntry { isa: Isa::SRLW,    decoder: Instruction::decode_type_r, execute: Self::SRLW,    disassemble: Self::disassemble_SRLW },
        IsaEntry { isa: Isa::SUBW,    decoder: Instruction::decode_type_r, execute: Self::SUBW,    disassemble: Self::disassemble_SUBW },
    ];
}
