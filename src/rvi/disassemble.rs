use crate::common::{isa::*, types::*};
use crate::rvi::*;

impl<U: Unsigned<S>, S: Signed<U>, const N: usize> DisassembleI32<U, S, N> for RVI<U, S, N> {
    fn load_disassemble_i32(&mut self) {
        self.disassemble[ISA::ADD as usize..=ISA::XORI as usize].copy_from_slice(&Self::DISASSEMBLE_I32);
    }

    fn disassemble_UNKNOWN(inst: Instruction<U, S>) -> String {
        format!("Unknown instruction {:?}", inst)
    }

    fn disassemble_ADD(inst: Instruction<U, S>) -> String {
        format!("ADD {:?}", inst)
    }

    fn disassemble_ADDI(inst: Instruction<U, S>) -> String {
        format!("ADDI {:?}", inst)
    }

    fn disassemble_AND(inst: Instruction<U, S>) -> String {
        format!("AND {:?}", inst)
    }

    fn disassemble_ANDI(inst: Instruction<U, S>) -> String {
        format!("ANDI {:?}", inst)
    }

    fn disassemble_AUIPC(inst: Instruction<U, S>) -> String {
        format!("AUIPC {:?}", inst)
    }

    fn disassemble_BEQ(inst: Instruction<U, S>) -> String {
        format!("BEQ {:?}", inst)
    }

    fn disassemble_BGE(inst: Instruction<U, S>) -> String {
        format!("BGE {:?}", inst)
    }

    fn disassemble_BGEU(inst: Instruction<U, S>) -> String {
        format!("BGEU {:?}", inst)
    }

    fn disassemble_BLT(inst: Instruction<U, S>) -> String {
        format!("BLT {:?}", inst)
    }

    fn disassemble_BLTU(inst: Instruction<U, S>) -> String {
        format!("BLTU {:?}", inst)
    }

    fn disassemble_BNE(inst: Instruction<U, S>) -> String {
        format!("BNE {:?}", inst)
    }

    fn disassemble_EBREAK(inst: Instruction<U, S>) -> String {
        format!("EBREAK {:?}", inst)
    }

    fn disassemble_ECALL(inst: Instruction<U, S>) -> String {
        format!("ECALL {:?}", inst)
    }

    fn disassemble_FENCE(inst: Instruction<U, S>) -> String {
        format!("FENCE {:?}", inst)
    }

    fn disassemble_JAL(inst: Instruction<U, S>) -> String {
        format!("JAL {:?}", inst)
    }

    fn disassemble_JALR(inst: Instruction<U, S>) -> String {
        format!("JALR {:?}", inst)
    }

    fn disassemble_LB(inst: Instruction<U, S>) -> String {
        format!("LB {:?}", inst)
    }

    fn disassemble_LBU(inst: Instruction<U, S>) -> String {
        format!("LBU {:?}", inst)
    }

    fn disassemble_LH(inst: Instruction<U, S>) -> String {
        format!("LH {:?}", inst)
    }

    fn disassemble_LHU(inst: Instruction<U, S>) -> String {
        format!("LHU {:?}", inst)
    }

    fn disassemble_LUI(inst: Instruction<U, S>) -> String {
        format!("LUI {:?}", inst)
    }

    fn disassemble_LW(inst: Instruction<U, S>) -> String {
        format!("LW {:?}", inst)
    }

    fn disassemble_OR(inst: Instruction<U, S>) -> String {
        format!("OR {:?}", inst)
    }

    fn disassemble_ORI(inst: Instruction<U, S>) -> String {
        format!("ORI {:?}", inst)
    }

    fn disassemble_SB(inst: Instruction<U, S>) -> String {
        format!("SB {:?}", inst)
    }

    fn disassemble_SH(inst: Instruction<U, S>) -> String {
        format!("SH {:?}", inst)
    }

    fn disassemble_SLL(inst: Instruction<U, S>) -> String {
        format!("SLL {:?}", inst)
    }

    fn disassemble_SLLI(inst: Instruction<U, S>) -> String {
        format!("SLLI {:?}", inst)
    }

    fn disassemble_SLT(inst: Instruction<U, S>) -> String {
        format!("SLT {:?}", inst)
    }

    fn disassemble_SLTI(inst: Instruction<U, S>) -> String {
        format!("SLTI {:?}", inst)
    }

    fn disassemble_SLTIU(inst: Instruction<U, S>) -> String {
        format!("SLTIU {:?}", inst)
    }

    fn disassemble_SLTU(inst: Instruction<U, S>) -> String {
        format!("SLTU {:?}", inst)
    }

    fn disassemble_SRA(inst: Instruction<U, S>) -> String {
        format!("SRA {:?}", inst)
    }

    fn disassemble_SRAI(inst: Instruction<U, S>) -> String {
        format!("SRAI {:?}", inst)
    }

    fn disassemble_SRL(inst: Instruction<U, S>) -> String {
        format!("SRL {:?}", inst)
    }

    fn disassemble_SRLI(inst: Instruction<U, S>) -> String {
        format!("SRLI {:?}", inst)
    }

    fn disassemble_SUB(inst: Instruction<U, S>) -> String {
        format!("SUB {:?}", inst)
    }

    fn disassemble_SW(inst: Instruction<U, S>) -> String {
        format!("SW {:?}", inst)
    }

    fn disassemble_XOR(inst: Instruction<U, S>) -> String {
        format!("XOR {:?}", inst)
    }

    fn disassemble_XORI(inst: Instruction<U, S>) -> String {
        format!("XORI {:?}", inst)
    }
}

impl DisassembleI64<u64, i64> for RV64I {
    fn load_disassemble_i64(&mut self) {
        self.disassemble[ISA::ADDIW as usize..=ISA::SUBW as usize].copy_from_slice(&Self::DISASSEMBLE_I64);
    }

    fn disassemble_ADDIW(inst: Instruction64) -> String {
        format!("ADDIW {:?}", inst)
    }

    fn disassemble_ADDW(inst: Instruction64) -> String {
        format!("ADDW {:?}", inst)
    }

    fn disassemble_LD(inst: Instruction64) -> String {
        format!("LD {:?}", inst)
    }

    fn disassemble_LWU(inst: Instruction64) -> String {
        format!("LWU {:?}", inst)
    }

    fn disassemble_SD(inst: Instruction64) -> String {
        format!("SD {:?}", inst)
    }

    fn disassemble_SLLIW(inst: Instruction64) -> String {
        format!("SLLIW {:?}", inst)
    }

    fn disassemble_SLLW(inst: Instruction64) -> String {
        format!("SLLW {:?}", inst)
    }

    fn disassemble_SRAIW(inst: Instruction64) -> String {
        format!("SRAIW {:?}", inst)
    }

    fn disassemble_SRAW(inst: Instruction64) -> String {
        format!("SRAW {:?}", inst)
    }

    fn disassemble_SRLIW(inst: Instruction64) -> String {
        format!("SRLIW {:?}", inst)
    }

    fn disassemble_SRLW(inst: Instruction64) -> String {
        format!("SRLW {:?}", inst)
    }

    fn disassemble_SUBW(inst: Instruction64) -> String {
        format!("SUBW {:?}", inst)
    }
}
