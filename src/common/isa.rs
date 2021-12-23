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

    DIV, // M32
    DIVU,
    MUL,
    MULH,
    MULHSU,
    MULHU,
    REM,
    REMU,

    DIVUW, // M64
    DIVW,
    MULW,
    REMUW,
    REMW,

    /// used internally by dyriscvic, not a real instruction
    _SIZE,
}

macro_rules! declare_isa_entry {
    ($entry:ident, $exec:ident, $pctype:ty, $inst:ty) => {
        /// This struct is meant to built a constexpr look-up table for all the ISA.
        pub struct $entry<EEI: ExecutionEnvironmentInterface> {
            /// The ISA itself.
            pub isa: Isa,
            /// The decoding instruction.
            pub decoder: fn(Isa, $pctype, u32) -> $inst,
            /// The execute method.
            pub execute: fn(&mut $exec<EEI>),
            /// The disassemble method.
            pub disassemble: fn($inst, bool) -> String,
        }

        impl<EEI: ExecutionEnvironmentInterface> std::marker::Copy for $entry<EEI> {}

        impl<EEI: ExecutionEnvironmentInterface> core::clone::Clone for $entry<EEI> {
            fn clone(&self) -> Self { *self }
        }
    };
}

declare_isa_entry!(IsaEntry32, RV32I, u32, Instruction32);
declare_isa_entry!(IsaEntry64, RV64I, u64, Instruction64);

macro_rules! declare_isa_lut {
    ($entry:ident, $exec:ident, $inst:ty) => {
        impl<EEI: ExecutionEnvironmentInterface> $exec<EEI> {
            /// Default empty IsaEntry for custom LUT initialization.
            pub const ISA_UNKNOWN: $entry<EEI> = $entry { isa: Isa::UNKNOWN, decoder: <$inst>::empty, execute: Self::UNKNOWN, disassemble: Self::disassemble_UNKNOWN };

            /// Maps an Isa to its associated decoding, executing and disassembling functions.
            pub const ISA_LUT_I32: [$entry<EEI>; Isa::XORI as usize] = [
                $entry { isa: Isa::ADD,     decoder: <$inst>::decode_type_r, execute: Self::ADD,     disassemble: Self::disassemble_ADD },
                $entry { isa: Isa::ADDI,    decoder: <$inst>::decode_type_i, execute: Self::ADDI,    disassemble: Self::disassemble_ADDI },
                $entry { isa: Isa::AND,     decoder: <$inst>::decode_type_r, execute: Self::AND,     disassemble: Self::disassemble_AND },
                $entry { isa: Isa::ANDI,    decoder: <$inst>::decode_type_i, execute: Self::ANDI,    disassemble: Self::disassemble_ANDI },
                $entry { isa: Isa::AUIPC,   decoder: <$inst>::decode_type_u, execute: Self::AUIPC,   disassemble: Self::disassemble_AUIPC },
                $entry { isa: Isa::BEQ,     decoder: <$inst>::decode_type_b, execute: Self::BEQ,     disassemble: Self::disassemble_BEQ },
                $entry { isa: Isa::BGE,     decoder: <$inst>::decode_type_b, execute: Self::BGE,     disassemble: Self::disassemble_BGE },
                $entry { isa: Isa::BGEU,    decoder: <$inst>::decode_type_b, execute: Self::BGEU,    disassemble: Self::disassemble_BGEU },
                $entry { isa: Isa::BLT,     decoder: <$inst>::decode_type_b, execute: Self::BLT,     disassemble: Self::disassemble_BLT },
                $entry { isa: Isa::BLTU,    decoder: <$inst>::decode_type_b, execute: Self::BLTU,    disassemble: Self::disassemble_BLTU },
                $entry { isa: Isa::BNE,     decoder: <$inst>::decode_type_b, execute: Self::BNE,     disassemble: Self::disassemble_BNE },
                $entry { isa: Isa::EBREAK,  decoder: <$inst>::empty,         execute: Self::EBREAK,  disassemble: Self::disassemble_EBREAK },
                $entry { isa: Isa::ECALL,   decoder: <$inst>::empty,         execute: Self::ECALL,   disassemble: Self::disassemble_ECALL },
                $entry { isa: Isa::FENCE,   decoder: <$inst>::decode_type_i, execute: Self::FENCE,   disassemble: Self::disassemble_FENCE },
                $entry { isa: Isa::JAL,     decoder: <$inst>::decode_type_j, execute: Self::JAL,     disassemble: Self::disassemble_JAL },
                $entry { isa: Isa::JALR,    decoder: <$inst>::decode_type_i, execute: Self::JALR,    disassemble: Self::disassemble_JALR },
                $entry { isa: Isa::LB,      decoder: <$inst>::decode_type_i, execute: Self::LB,      disassemble: Self::disassemble_LB },
                $entry { isa: Isa::LBU,     decoder: <$inst>::decode_type_i, execute: Self::LBU,     disassemble: Self::disassemble_LBU },
                $entry { isa: Isa::LH,      decoder: <$inst>::decode_type_i, execute: Self::LH,      disassemble: Self::disassemble_LH },
                $entry { isa: Isa::LHU,     decoder: <$inst>::decode_type_i, execute: Self::LHU,     disassemble: Self::disassemble_LHU },
                $entry { isa: Isa::LUI,     decoder: <$inst>::decode_type_u, execute: Self::LUI,     disassemble: Self::disassemble_LUI },
                $entry { isa: Isa::LW,      decoder: <$inst>::decode_type_i, execute: Self::LW,      disassemble: Self::disassemble_LW },
                $entry { isa: Isa::OR,      decoder: <$inst>::decode_type_r, execute: Self::OR,      disassemble: Self::disassemble_OR },
                $entry { isa: Isa::ORI,     decoder: <$inst>::decode_type_i, execute: Self::ORI,     disassemble: Self::disassemble_ORI },
                $entry { isa: Isa::SB,      decoder: <$inst>::decode_type_s, execute: Self::SB,      disassemble: Self::disassemble_SB },
                $entry { isa: Isa::SH,      decoder: <$inst>::decode_type_s, execute: Self::SH,      disassemble: Self::disassemble_SH },
                $entry { isa: Isa::SLL,     decoder: <$inst>::decode_type_r, execute: Self::SLL,     disassemble: Self::disassemble_SLL },
                $entry { isa: Isa::SLLI,    decoder: <$inst>::decode_type_i, execute: Self::SLLI,    disassemble: Self::disassemble_SLLI },
                $entry { isa: Isa::SLT,     decoder: <$inst>::decode_type_r, execute: Self::SLT,     disassemble: Self::disassemble_SLT },
                $entry { isa: Isa::SLTI,    decoder: <$inst>::decode_type_i, execute: Self::SLTI,    disassemble: Self::disassemble_SLTI },
                $entry { isa: Isa::SLTIU,   decoder: <$inst>::decode_type_i, execute: Self::SLTIU,   disassemble: Self::disassemble_SLTIU },
                $entry { isa: Isa::SLTU,    decoder: <$inst>::decode_type_r, execute: Self::SLTU,    disassemble: Self::disassemble_SLTU },
                $entry { isa: Isa::SRA,     decoder: <$inst>::decode_type_r, execute: Self::SRA,     disassemble: Self::disassemble_SRA },
                $entry { isa: Isa::SRAI,    decoder: <$inst>::decode_type_i, execute: Self::SRAI,    disassemble: Self::disassemble_SRAI },
                $entry { isa: Isa::SRL,     decoder: <$inst>::decode_type_r, execute: Self::SRL,     disassemble: Self::disassemble_SRL },
                $entry { isa: Isa::SRLI,    decoder: <$inst>::decode_type_i, execute: Self::SRLI,    disassemble: Self::disassemble_SRLI },
                $entry { isa: Isa::SUB,     decoder: <$inst>::decode_type_r, execute: Self::SUB,     disassemble: Self::disassemble_SUB },
                $entry { isa: Isa::SW,      decoder: <$inst>::decode_type_s, execute: Self::SW,      disassemble: Self::disassemble_SW },
                $entry { isa: Isa::XOR,     decoder: <$inst>::decode_type_r, execute: Self::XOR,     disassemble: Self::disassemble_XOR },
                $entry { isa: Isa::XORI,    decoder: <$inst>::decode_type_i, execute: Self::XORI,    disassemble: Self::disassemble_XORI },
            ];

            /// Maps an Isa to its associated decoding, executing and disassembling functions.
            pub const ISA_LUT_M32: [$entry<EEI>; Isa::REMU as usize + 1 - Isa::DIV as usize] = [
                $entry { isa: Isa::DIV,    decoder: <$inst>::decode_type_r, execute: Self::DIV,    disassemble: Self::disassemble_DIV },
                $entry { isa: Isa::DIVU,   decoder: <$inst>::decode_type_r, execute: Self::DIVU,   disassemble: Self::disassemble_DIVU },
                $entry { isa: Isa::MUL,    decoder: <$inst>::decode_type_r, execute: Self::MUL,    disassemble: Self::disassemble_MUL },
                $entry { isa: Isa::MULH,   decoder: <$inst>::decode_type_r, execute: Self::MULH,   disassemble: Self::disassemble_MULH },
                $entry { isa: Isa::MULHSU, decoder: <$inst>::decode_type_r, execute: Self::MULHSU, disassemble: Self::disassemble_MULHSU },
                $entry { isa: Isa::MULHU,  decoder: <$inst>::decode_type_r, execute: Self::MULHU,  disassemble: Self::disassemble_MULHU },
                $entry { isa: Isa::REM,    decoder: <$inst>::decode_type_r, execute: Self::REM,    disassemble: Self::disassemble_REM },
                $entry { isa: Isa::REMU,   decoder: <$inst>::decode_type_r, execute: Self::REMU,    disassemble: Self::disassemble_REMU },
            ];
        }
    };
}

impl<EEI: ExecutionEnvironmentInterface> RV64I<EEI> {
    /// Maps an Isa to its associated decoding, executing and disassembling functions.
    pub const ISA_LUT_I64: [IsaEntry64<EEI>; Isa::SUBW as usize + 1 - Isa::ADDIW as usize] = [
        IsaEntry64 { isa: Isa::ADDIW,   decoder: Instruction64::decode_type_i, execute: Self::ADDIW,   disassemble: Self::disassemble_ADDIW },
        IsaEntry64 { isa: Isa::ADDW,    decoder: Instruction64::decode_type_r, execute: Self::ADDW,    disassemble: Self::disassemble_ADDW },
        IsaEntry64 { isa: Isa::LD,      decoder: Instruction64::decode_type_i, execute: Self::LD,      disassemble: Self::disassemble_LD },
        IsaEntry64 { isa: Isa::LWU,     decoder: Instruction64::decode_type_i, execute: Self::LWU,     disassemble: Self::disassemble_LWU },
        IsaEntry64 { isa: Isa::SD,      decoder: Instruction64::decode_type_s, execute: Self::SD,      disassemble: Self::disassemble_SD },
        IsaEntry64 { isa: Isa::SLLIW,   decoder: Instruction64::decode_type_i, execute: Self::SLLIW,   disassemble: Self::disassemble_SLLIW }, // type i or type r ?
        IsaEntry64 { isa: Isa::SLLW,    decoder: Instruction64::decode_type_r, execute: Self::SLLW,    disassemble: Self::disassemble_SLLW },
        IsaEntry64 { isa: Isa::SRAIW,   decoder: Instruction64::decode_type_i, execute: Self::SRAIW,   disassemble: Self::disassemble_SRAIW }, // type i or type r ?
        IsaEntry64 { isa: Isa::SRAW,    decoder: Instruction64::decode_type_r, execute: Self::SRAW,    disassemble: Self::disassemble_SRAW },
        IsaEntry64 { isa: Isa::SRLIW,   decoder: Instruction64::decode_type_i, execute: Self::SRLIW,   disassemble: Self::disassemble_SRLIW }, // type i or type r ?
        IsaEntry64 { isa: Isa::SRLW,    decoder: Instruction64::decode_type_r, execute: Self::SRLW,    disassemble: Self::disassemble_SRLW },
        IsaEntry64 { isa: Isa::SUBW,    decoder: Instruction64::decode_type_r, execute: Self::SUBW,    disassemble: Self::disassemble_SUBW },
    ];

    pub const ISA_LUT_M64: [IsaEntry64<EEI>; Isa::REMW as usize + 1 - Isa::DIVUW as usize] = [
        IsaEntry64 { isa: Isa::DIVUW, decoder: Instruction64::decode_type_r, execute: Self::DIVUW, disassemble: Self::disassemble_DIVUW },
        IsaEntry64 { isa: Isa::DIVW,  decoder: Instruction64::decode_type_r, execute: Self::DIVW,  disassemble: Self::disassemble_DIVW },
        IsaEntry64 { isa: Isa::MULW,  decoder: Instruction64::decode_type_r, execute: Self::MULW,  disassemble: Self::disassemble_MULW },
        IsaEntry64 { isa: Isa::REMUW, decoder: Instruction64::decode_type_r, execute: Self::REMUW, disassemble: Self::disassemble_REMUW },
        IsaEntry64 { isa: Isa::REMW,  decoder: Instruction64::decode_type_r, execute: Self::REMW,  disassemble: Self::disassemble_REMW },
    ];
}

declare_isa_lut!(IsaEntry32, RV32I, Instruction32);
declare_isa_lut!(IsaEntry64, RV64I, Instruction64);
