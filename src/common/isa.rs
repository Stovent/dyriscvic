use crate::common::{instruction::*, types::*};
use crate::public::*;
use crate::rvi::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISA {
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

pub trait I32<U: Unsigned<S>, S: Signed<U>, EEI: ExecutionEnvironmentInterface<U>, const N: usize> {
    const EXECUTE_I32: [fn(&mut RVI<U, S, EEI, N>); 40] = [
        RVI::<U, S, EEI, N>::ADD,
        RVI::<U, S, EEI, N>::ADDI,
        RVI::<U, S, EEI, N>::AND,
        RVI::<U, S, EEI, N>::ANDI,
        RVI::<U, S, EEI, N>::AUIPC,
        RVI::<U, S, EEI, N>::BEQ,
        RVI::<U, S, EEI, N>::BGE,
        RVI::<U, S, EEI, N>::BGEU,
        RVI::<U, S, EEI, N>::BLT,
        RVI::<U, S, EEI, N>::BLTU,
        RVI::<U, S, EEI, N>::BNE,
        RVI::<U, S, EEI, N>::EBREAK,
        RVI::<U, S, EEI, N>::ECALL,
        RVI::<U, S, EEI, N>::FENCE,
        RVI::<U, S, EEI, N>::JAL,
        RVI::<U, S, EEI, N>::JALR,
        RVI::<U, S, EEI, N>::LB,
        RVI::<U, S, EEI, N>::LBU,
        RVI::<U, S, EEI, N>::LH,
        RVI::<U, S, EEI, N>::LHU,
        RVI::<U, S, EEI, N>::LUI,
        RVI::<U, S, EEI, N>::LW,
        RVI::<U, S, EEI, N>::OR,
        RVI::<U, S, EEI, N>::ORI,
        RVI::<U, S, EEI, N>::SB,
        RVI::<U, S, EEI, N>::SH,
        RVI::<U, S, EEI, N>::SLL,
        RVI::<U, S, EEI, N>::SLLI,
        RVI::<U, S, EEI, N>::SLT,
        RVI::<U, S, EEI, N>::SLTI,
        RVI::<U, S, EEI, N>::SLTIU,
        RVI::<U, S, EEI, N>::SLTU,
        RVI::<U, S, EEI, N>::SRA,
        RVI::<U, S, EEI, N>::SRAI,
        RVI::<U, S, EEI, N>::SRL,
        RVI::<U, S, EEI, N>::SRLI,
        RVI::<U, S, EEI, N>::SUB,
        RVI::<U, S, EEI, N>::SW,
        RVI::<U, S, EEI, N>::XOR,
        RVI::<U, S, EEI, N>::XORI,
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

pub trait DisassembleI32<U: Unsigned<S>, S: Signed<U>, EEI: ExecutionEnvironmentInterface<U>, const N: usize> {
    const DISASSEMBLE_I32: [fn(Instruction<U, S>, bool) -> String; 40] = [
        RVI::<U, S, EEI, N>::disassemble_ADD,
        RVI::<U, S, EEI, N>::disassemble_ADDI,
        RVI::<U, S, EEI, N>::disassemble_AND,
        RVI::<U, S, EEI, N>::disassemble_ANDI,
        RVI::<U, S, EEI, N>::disassemble_AUIPC,
        RVI::<U, S, EEI, N>::disassemble_BEQ,
        RVI::<U, S, EEI, N>::disassemble_BGE,
        RVI::<U, S, EEI, N>::disassemble_BGEU,
        RVI::<U, S, EEI, N>::disassemble_BLT,
        RVI::<U, S, EEI, N>::disassemble_BLTU,
        RVI::<U, S, EEI, N>::disassemble_BNE,
        RVI::<U, S, EEI, N>::disassemble_EBREAK,
        RVI::<U, S, EEI, N>::disassemble_ECALL,
        RVI::<U, S, EEI, N>::disassemble_FENCE,
        RVI::<U, S, EEI, N>::disassemble_JAL,
        RVI::<U, S, EEI, N>::disassemble_JALR,
        RVI::<U, S, EEI, N>::disassemble_LB,
        RVI::<U, S, EEI, N>::disassemble_LBU,
        RVI::<U, S, EEI, N>::disassemble_LH,
        RVI::<U, S, EEI, N>::disassemble_LHU,
        RVI::<U, S, EEI, N>::disassemble_LUI,
        RVI::<U, S, EEI, N>::disassemble_LW,
        RVI::<U, S, EEI, N>::disassemble_OR,
        RVI::<U, S, EEI, N>::disassemble_ORI,
        RVI::<U, S, EEI, N>::disassemble_SB,
        RVI::<U, S, EEI, N>::disassemble_SH,
        RVI::<U, S, EEI, N>::disassemble_SLL,
        RVI::<U, S, EEI, N>::disassemble_SLLI,
        RVI::<U, S, EEI, N>::disassemble_SLT,
        RVI::<U, S, EEI, N>::disassemble_SLTI,
        RVI::<U, S, EEI, N>::disassemble_SLTIU,
        RVI::<U, S, EEI, N>::disassemble_SLTU,
        RVI::<U, S, EEI, N>::disassemble_SRA,
        RVI::<U, S, EEI, N>::disassemble_SRAI,
        RVI::<U, S, EEI, N>::disassemble_SRL,
        RVI::<U, S, EEI, N>::disassemble_SRLI,
        RVI::<U, S, EEI, N>::disassemble_SUB,
        RVI::<U, S, EEI, N>::disassemble_SW,
        RVI::<U, S, EEI, N>::disassemble_XOR,
        RVI::<U, S, EEI, N>::disassemble_XORI,
    ];
    fn load_disassemble_i32(&mut self);
    fn disassemble_UNKNOWN(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_ADD(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_ADDI(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_AND(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_ANDI(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_AUIPC(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_BEQ(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_BGE(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_BGEU(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_BLT(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_BLTU(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_BNE(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_EBREAK(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_ECALL(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_FENCE(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_JAL(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_JALR(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_LB(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_LBU(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_LH(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_LHU(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_LUI(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_LW(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_OR(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_ORI(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SB(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SH(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SLL(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SLLI(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SLT(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SLTI(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SLTIU(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SLTU(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SRA(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SRAI(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SRL(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SRLI(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SUB(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SW(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_XOR(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_XORI(inst: Instruction<U, S>, abi_name: bool) -> String;
}

pub trait I64 {
    fn load_execute_i64(&mut self);
    fn ADDIW(&mut self);
    fn ADDW(&mut self);
    fn LD(&mut self);
    fn LWU(&mut self);
    fn SD(&mut self);
    fn SLLIW(&mut self);
    fn SLLW(&mut self);
    fn SRAIW(&mut self);
    fn SRAW(&mut self);
    fn SRLIW(&mut self);
    fn SRLW(&mut self);
    fn SUBW(&mut self);
}

pub trait DisassembleI64<U: Unsigned<S>, S: Signed<U>> {
    fn load_disassemble_i64(&mut self);
    fn disassemble_ADDIW(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_ADDW(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_LD(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_LWU(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SD(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SLLIW(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SLLW(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SRAIW(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SRAW(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SRLIW(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SRLW(inst: Instruction<U, S>, abi_name: bool) -> String;
    fn disassemble_SUBW(inst: Instruction<U, S>, abi_name: bool) -> String;
}

impl<EEI: ExecutionEnvironmentInterface<u64>> RV64I<EEI> {
    pub const EXECUTE_I64: [fn(&mut RV64I<EEI>); 12] = [
        Self::ADDIW,
        Self::ADDW,
        Self::LD,
        Self::LWU,
        Self::SD,
        Self::SLLIW,
        Self::SLLW,
        Self::SRAIW,
        Self::SRAW,
        Self::SRLIW,
        Self::SRLW,
        Self::SUBW,
    ];

    pub const DISASSEMBLE_I64: [fn(inst: Instruction64, abi_name: bool) -> String; 12] = [
        Self::disassemble_ADDIW,
        Self::disassemble_ADDW,
        Self::disassemble_LD,
        Self::disassemble_LWU,
        Self::disassemble_SD,
        Self::disassemble_SLLIW,
        Self::disassemble_SLLW,
        Self::disassemble_SRAIW,
        Self::disassemble_SRAW,
        Self::disassemble_SRLIW,
        Self::disassemble_SRLW,
        Self::disassemble_SUBW,
    ];
}
