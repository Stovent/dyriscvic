use crate::common::{instruction::*, types::*};
use crate::rvi::*;

pub trait I32<U: Unsigned<S>, S: Signed<U>, const N: usize> {
    const EXECUTE_I32: [fn(&mut RVI<U, S, N>); 40] = [
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

pub trait DisassembleI32<U: Unsigned<S>, S: Signed<U>, const N: usize> {
    const DISASSEMBLE_I32: [fn(Instruction<U, S>); 40] = [
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
    fn load_disassemble_i32(&mut self);
    fn disassemble_UNKNOWN(inst: Instruction<U, S>);
    fn disassemble_ADD(inst: Instruction<U, S>);
    fn disassemble_ADDI(inst: Instruction<U, S>);
    fn disassemble_AND(inst: Instruction<U, S>);
    fn disassemble_ANDI(inst: Instruction<U, S>);
    fn disassemble_AUIPC(inst: Instruction<U, S>);
    fn disassemble_BEQ(inst: Instruction<U, S>);
    fn disassemble_BGE(inst: Instruction<U, S>);
    fn disassemble_BGEU(inst: Instruction<U, S>);
    fn disassemble_BLT(inst: Instruction<U, S>);
    fn disassemble_BLTU(inst: Instruction<U, S>);
    fn disassemble_BNE(inst: Instruction<U, S>);
    fn disassemble_EBREAK(inst: Instruction<U, S>);
    fn disassemble_ECALL(inst: Instruction<U, S>);
    fn disassemble_FENCE(inst: Instruction<U, S>);
    fn disassemble_JAL(inst: Instruction<U, S>);
    fn disassemble_JALR(inst: Instruction<U, S>);
    fn disassemble_LB(inst: Instruction<U, S>);
    fn disassemble_LBU(inst: Instruction<U, S>);
    fn disassemble_LH(inst: Instruction<U, S>);
    fn disassemble_LHU(inst: Instruction<U, S>);
    fn disassemble_LUI(inst: Instruction<U, S>);
    fn disassemble_LW(inst: Instruction<U, S>);
    fn disassemble_OR(inst: Instruction<U, S>);
    fn disassemble_ORI(inst: Instruction<U, S>);
    fn disassemble_SB(inst: Instruction<U, S>);
    fn disassemble_SH(inst: Instruction<U, S>);
    fn disassemble_SLL(inst: Instruction<U, S>);
    fn disassemble_SLLI(inst: Instruction<U, S>);
    fn disassemble_SLT(inst: Instruction<U, S>);
    fn disassemble_SLTI(inst: Instruction<U, S>);
    fn disassemble_SLTIU(inst: Instruction<U, S>);
    fn disassemble_SLTU(inst: Instruction<U, S>);
    fn disassemble_SRA(inst: Instruction<U, S>);
    fn disassemble_SRAI(inst: Instruction<U, S>);
    fn disassemble_SRL(inst: Instruction<U, S>);
    fn disassemble_SRLI(inst: Instruction<U, S>);
    fn disassemble_SUB(inst: Instruction<U, S>);
    fn disassemble_SW(inst: Instruction<U, S>);
    fn disassemble_XOR(inst: Instruction<U, S>);
    fn disassemble_XORI(inst: Instruction<U, S>);
}
