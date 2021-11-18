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
pub struct IsaEntry<EEI: ExecutionEnvironmentInterface> {
    /// The ISA itself.
    pub isa: Isa,
    /// The decoding instruction.
    pub decoder: fn(Isa, u64, u32) -> Instruction,
    /// The execute method.
    pub execute: fn(&mut RV64I<EEI>),
    // /// The disassemble method.
    // pub disassemble: fn(Instruction, bool) -> String,
}

// impl<EEI: ExecutionEnvironmentInterface> Isa {
//     /// Maps an Isa to its associated decoding, executing and disassembling functions.
//     pub const ISA_LUT: [IsaEntry<EEI>; 2] = [
//         IsaEntry { isa: Isa::UNKNOWN, execute: RV64I::UNKNOWN, decoder: Instruction::empty },
//         IsaEntry { isa: Isa::ADD, execute: RV64I::ADD, decoder: Instruction::decode_type_r },
//     ];
// }

pub trait I32<EEI: ExecutionEnvironmentInterface> {
    const EXECUTE_I32: [fn(&mut RV64I<EEI>); 40] = [
        RV64I::<EEI>::ADD,
        RV64I::<EEI>::ADDI,
        RV64I::<EEI>::AND,
        RV64I::<EEI>::ANDI,
        RV64I::<EEI>::AUIPC,
        RV64I::<EEI>::BEQ,
        RV64I::<EEI>::BGE,
        RV64I::<EEI>::BGEU,
        RV64I::<EEI>::BLT,
        RV64I::<EEI>::BLTU,
        RV64I::<EEI>::BNE,
        RV64I::<EEI>::EBREAK,
        RV64I::<EEI>::ECALL,
        RV64I::<EEI>::FENCE,
        RV64I::<EEI>::JAL,
        RV64I::<EEI>::JALR,
        RV64I::<EEI>::LB,
        RV64I::<EEI>::LBU,
        RV64I::<EEI>::LH,
        RV64I::<EEI>::LHU,
        RV64I::<EEI>::LUI,
        RV64I::<EEI>::LW,
        RV64I::<EEI>::OR,
        RV64I::<EEI>::ORI,
        RV64I::<EEI>::SB,
        RV64I::<EEI>::SH,
        RV64I::<EEI>::SLL,
        RV64I::<EEI>::SLLI,
        RV64I::<EEI>::SLT,
        RV64I::<EEI>::SLTI,
        RV64I::<EEI>::SLTIU,
        RV64I::<EEI>::SLTU,
        RV64I::<EEI>::SRA,
        RV64I::<EEI>::SRAI,
        RV64I::<EEI>::SRL,
        RV64I::<EEI>::SRLI,
        RV64I::<EEI>::SUB,
        RV64I::<EEI>::SW,
        RV64I::<EEI>::XOR,
        RV64I::<EEI>::XORI,
    ];
    fn load_execute_i32(&mut self);
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

pub trait DisassembleI32<EEI: ExecutionEnvironmentInterface> {
    const DISASSEMBLE_I32: [fn(Instruction, bool) -> String; 40] = [
        RV64I::<EEI>::disassemble_ADD,
        RV64I::<EEI>::disassemble_ADDI,
        RV64I::<EEI>::disassemble_AND,
        RV64I::<EEI>::disassemble_ANDI,
        RV64I::<EEI>::disassemble_AUIPC,
        RV64I::<EEI>::disassemble_BEQ,
        RV64I::<EEI>::disassemble_BGE,
        RV64I::<EEI>::disassemble_BGEU,
        RV64I::<EEI>::disassemble_BLT,
        RV64I::<EEI>::disassemble_BLTU,
        RV64I::<EEI>::disassemble_BNE,
        RV64I::<EEI>::disassemble_EBREAK,
        RV64I::<EEI>::disassemble_ECALL,
        RV64I::<EEI>::disassemble_FENCE,
        RV64I::<EEI>::disassemble_JAL,
        RV64I::<EEI>::disassemble_JALR,
        RV64I::<EEI>::disassemble_LB,
        RV64I::<EEI>::disassemble_LBU,
        RV64I::<EEI>::disassemble_LH,
        RV64I::<EEI>::disassemble_LHU,
        RV64I::<EEI>::disassemble_LUI,
        RV64I::<EEI>::disassemble_LW,
        RV64I::<EEI>::disassemble_OR,
        RV64I::<EEI>::disassemble_ORI,
        RV64I::<EEI>::disassemble_SB,
        RV64I::<EEI>::disassemble_SH,
        RV64I::<EEI>::disassemble_SLL,
        RV64I::<EEI>::disassemble_SLLI,
        RV64I::<EEI>::disassemble_SLT,
        RV64I::<EEI>::disassemble_SLTI,
        RV64I::<EEI>::disassemble_SLTIU,
        RV64I::<EEI>::disassemble_SLTU,
        RV64I::<EEI>::disassemble_SRA,
        RV64I::<EEI>::disassemble_SRAI,
        RV64I::<EEI>::disassemble_SRL,
        RV64I::<EEI>::disassemble_SRLI,
        RV64I::<EEI>::disassemble_SUB,
        RV64I::<EEI>::disassemble_SW,
        RV64I::<EEI>::disassemble_XOR,
        RV64I::<EEI>::disassemble_XORI,
    ];
    fn load_disassemble_i32(&mut self);
    fn disassemble_UNKNOWN(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_ADD(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_ADDI(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_AND(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_ANDI(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_AUIPC(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_BEQ(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_BGE(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_BGEU(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_BLT(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_BLTU(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_BNE(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_EBREAK(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_ECALL(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_FENCE(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_JAL(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_JALR(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_LB(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_LBU(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_LH(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_LHU(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_LUI(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_LW(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_OR(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_ORI(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SB(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SH(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SLL(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SLLI(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SLT(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SLTI(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SLTIU(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SLTU(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SRA(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SRAI(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SRL(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SRLI(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SUB(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_SW(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_XOR(inst: Instruction, abi_name: bool) -> String;
    fn disassemble_XORI(inst: Instruction, abi_name: bool) -> String;
}
