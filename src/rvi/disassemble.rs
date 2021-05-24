use crate::common::{extensions::*, types::*};
use crate::rvi::*;

impl<U: Unsigned<S>, S: Signed<U>, const N: usize> DisassembleI32<U, S, N> for RVI<U, S, N> {
    fn load_disassemble_i32(&mut self) {
        self.disassemble[ISA::ADD as usize..=ISA::XORI as usize].copy_from_slice(&Self::DISASSEMBLE_I32);
    }

    fn disassemble_UNKNOWN(inst: Instruction<U, S>) {
        println!("Error: unknown instruction {:?}", inst);
    }

    fn disassemble_ADD(inst: Instruction<U, S>) {
        println!("Instruction: ADD {:?}", inst);
    }

    fn disassemble_ADDI(inst: Instruction<U, S>) {
        println!("Instruction: ADDI {:?}", inst);
    }

    fn disassemble_AND(inst: Instruction<U, S>) {
        println!("Instruction: AND {:?}", inst);
    }

    fn disassemble_ANDI(inst: Instruction<U, S>) {
        println!("Instruction: ANDI {:?}", inst);
    }

    fn disassemble_AUIPC(inst: Instruction<U, S>) {
        println!("Instruction: AUIPC {:?}", inst);
    }

    fn disassemble_BEQ(inst: Instruction<U, S>) {
        println!("Instruction: BEQ {:?}", inst);
    }

    fn disassemble_BGE(inst: Instruction<U, S>) {
        println!("Instruction: BGE {:?}", inst);
    }

    fn disassemble_BGEU(inst: Instruction<U, S>) {
        println!("Instruction: BGEU {:?}", inst);
    }

    fn disassemble_BLT(inst: Instruction<U, S>) {
        println!("Instruction: BLT {:?}", inst);
    }

    fn disassemble_BLTU(inst: Instruction<U, S>) {
        println!("Instruction: BLTU {:?}", inst);
    }

    fn disassemble_BNE(inst: Instruction<U, S>) {
        println!("Instruction: BNE {:?}", inst);
    }

    fn disassemble_EBREAK(inst: Instruction<U, S>) {
        println!("Instruction: EBREAK {:?}", inst);
    }

    fn disassemble_ECALL(inst: Instruction<U, S>) {
        println!("Instruction: ECALL {:?}", inst);
    }

    fn disassemble_FENCE(inst: Instruction<U, S>) {
        println!("Instruction: FENCE {:?}", inst);
    }

    fn disassemble_JAL(inst: Instruction<U, S>) {
        println!("Instruction: JAL {:?}", inst);
    }

    fn disassemble_JALR(inst: Instruction<U, S>) {
        println!("Instruction: JALR {:?}", inst);
    }

    fn disassemble_LB(inst: Instruction<U, S>) {
        println!("Instruction: LB {:?}", inst);
    }

    fn disassemble_LBU(inst: Instruction<U, S>) {
        println!("Instruction: LBU {:?}", inst);
    }

    fn disassemble_LH(inst: Instruction<U, S>) {
        println!("Instruction: LH {:?}", inst);
    }

    fn disassemble_LHU(inst: Instruction<U, S>) {
        println!("Instruction: LHU {:?}", inst);
    }

    fn disassemble_LUI(inst: Instruction<U, S>) {
        println!("Instruction: LUI {:?}", inst);
    }

    fn disassemble_LW(inst: Instruction<U, S>) {
        println!("Instruction: LW {:?}", inst);
    }

    fn disassemble_OR(inst: Instruction<U, S>) {
        println!("Instruction: OR {:?}", inst);
    }

    fn disassemble_ORI(inst: Instruction<U, S>) {
        println!("Instruction: ORI {:?}", inst);
    }

    fn disassemble_SB(inst: Instruction<U, S>) {
        println!("Instruction: SB {:?}", inst);
    }

    fn disassemble_SH(inst: Instruction<U, S>) {
        println!("Instruction: SH {:?}", inst);
    }

    fn disassemble_SLL(inst: Instruction<U, S>) {
        println!("Instruction: SLL {:?}", inst);
    }

    fn disassemble_SLLI(inst: Instruction<U, S>) {
        println!("Instruction: SLLI {:?}", inst);
    }

    fn disassemble_SLT(inst: Instruction<U, S>) {
        println!("Instruction: SLT {:?}", inst);
    }

    fn disassemble_SLTI(inst: Instruction<U, S>) {
        println!("Instruction: SLTI {:?}", inst);
    }

    fn disassemble_SLTIU(inst: Instruction<U, S>) {
        println!("Instruction: SLTIU {:?}", inst);
    }

    fn disassemble_SLTU(inst: Instruction<U, S>) {
        println!("Instruction: SLTU {:?}", inst);
    }

    fn disassemble_SRA(inst: Instruction<U, S>) {
        println!("Instruction: SRA {:?}", inst);
    }

    fn disassemble_SRAI(inst: Instruction<U, S>) {
        println!("Instruction: SRAI {:?}", inst);
    }

    fn disassemble_SRL(inst: Instruction<U, S>) {
        println!("Instruction: SRL {:?}", inst);
    }

    fn disassemble_SRLI(inst: Instruction<U, S>) {
        println!("Instruction: SRLI {:?}", inst);
    }

    fn disassemble_SUB(inst: Instruction<U, S>) {
        println!("Instruction: SUB {:?}", inst);
    }

    fn disassemble_SW(inst: Instruction<U, S>) {
        println!("Instruction: SW {:?}", inst);
    }

    fn disassemble_XOR(inst: Instruction<U, S>) {
        println!("Instruction: XOR {:?}", inst);
    }

    fn disassemble_XORI(inst: Instruction<U, S>) {
        println!("Instruction: XORI {:?}", inst);
    }
}
