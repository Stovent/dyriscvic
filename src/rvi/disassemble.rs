use crate::common::{extensions::*, types::*};
use crate::rvi::*;

impl<'a, PC: Unsigned, IMM: Signed, const N: usize> DisassembleI<'a, PC, IMM, N> for RV32<'a, N> {
    fn disassemble_UNKNOWN(inst: Instruction<PC, IMM>) {
        println!("Error: unknown instruction {:?}", inst);
    }

    fn disassemble_ADD(inst: Instruction<PC, IMM>) {
        println!("Instruction: ADD {:?}", inst);
    }

    fn disassemble_ADDI(inst: Instruction<PC, IMM>) {
        println!("Instruction: ADDI {:?}", inst);
    }

    fn disassemble_AND(inst: Instruction<PC, IMM>) {
        println!("Instruction: AND {:?}", inst);
    }

    fn disassemble_ANDI(inst: Instruction<PC, IMM>) {
        println!("Instruction: ANDI {:?}", inst);
    }

    fn disassemble_AUIPC(inst: Instruction<PC, IMM>) {
        println!("Instruction: AUIPC {:?}", inst);
    }

    fn disassemble_BEQ(inst: Instruction<PC, IMM>) {
        println!("Instruction: BEQ {:?}", inst);
    }

    fn disassemble_BGE(inst: Instruction<PC, IMM>) {
        println!("Instruction: BGE {:?}", inst);
    }

    fn disassemble_BGEU(inst: Instruction<PC, IMM>) {
        println!("Instruction: BGEU {:?}", inst);
    }

    fn disassemble_BLT(inst: Instruction<PC, IMM>) {
        println!("Instruction: BLT {:?}", inst);
    }

    fn disassemble_BLTU(inst: Instruction<PC, IMM>) {
        println!("Instruction: BLTU {:?}", inst);
    }

    fn disassemble_BNE(inst: Instruction<PC, IMM>) {
        println!("Instruction: BNE {:?}", inst);
    }

    fn disassemble_EBREAK(inst: Instruction<PC, IMM>) {
        println!("Instruction: EBREAK {:?}", inst);
    }

    fn disassemble_ECALL(inst: Instruction<PC, IMM>) {
        println!("Instruction: ECALL {:?}", inst);
    }

    fn disassemble_FENCE(inst: Instruction<PC, IMM>) {
        println!("Instruction: FENCE {:?}", inst);
    }

    fn disassemble_JAL(inst: Instruction<PC, IMM>) {
        println!("Instruction: JAL {:?}", inst);
    }

    fn disassemble_JALR(inst: Instruction<PC, IMM>) {
        println!("Instruction: JALR {:?}", inst);
    }

    fn disassemble_LB(inst: Instruction<PC, IMM>) {
        println!("Instruction: LB {:?}", inst);
    }

    fn disassemble_LBU(inst: Instruction<PC, IMM>) {
        println!("Instruction: LBU {:?}", inst);
    }

    fn disassemble_LH(inst: Instruction<PC, IMM>) {
        println!("Instruction: LH {:?}", inst);
    }

    fn disassemble_LHU(inst: Instruction<PC, IMM>) {
        println!("Instruction: LHU {:?}", inst);
    }

    fn disassemble_LUI(inst: Instruction<PC, IMM>) {
        println!("Instruction: LUI {:?}", inst);
    }

    fn disassemble_LW(inst: Instruction<PC, IMM>) {
        println!("Instruction: LW {:?}", inst);
    }

    fn disassemble_OR(inst: Instruction<PC, IMM>) {
        println!("Instruction: OR {:?}", inst);
    }

    fn disassemble_ORI(inst: Instruction<PC, IMM>) {
        println!("Instruction: ORI {:?}", inst);
    }

    fn disassemble_SB(inst: Instruction<PC, IMM>) {
        println!("Instruction: SB {:?}", inst);
    }

    fn disassemble_SH(inst: Instruction<PC, IMM>) {
        println!("Instruction: SH {:?}", inst);
    }

    fn disassemble_SLL(inst: Instruction<PC, IMM>) {
        println!("Instruction: SLL {:?}", inst);
    }

    fn disassemble_SLLI(inst: Instruction<PC, IMM>) {
        println!("Instruction: SLLI {:?}", inst);
    }

    fn disassemble_SLT(inst: Instruction<PC, IMM>) {
        println!("Instruction: SLT {:?}", inst);
    }

    fn disassemble_SLTI(inst: Instruction<PC, IMM>) {
        println!("Instruction: SLTI {:?}", inst);
    }

    fn disassemble_SLTIU(inst: Instruction<PC, IMM>) {
        println!("Instruction: SLTIU {:?}", inst);
    }

    fn disassemble_SLTU(inst: Instruction<PC, IMM>) {
        println!("Instruction: SLTU {:?}", inst);
    }

    fn disassemble_SRA(inst: Instruction<PC, IMM>) {
        println!("Instruction: SRA {:?}", inst);
    }

    fn disassemble_SRAI(inst: Instruction<PC, IMM>) {
        println!("Instruction: SRAI {:?}", inst);
    }

    fn disassemble_SRL(inst: Instruction<PC, IMM>) {
        println!("Instruction: SRL {:?}", inst);
    }

    fn disassemble_SRLI(inst: Instruction<PC, IMM>) {
        println!("Instruction: SRLI {:?}", inst);
    }

    fn disassemble_SUB(inst: Instruction<PC, IMM>) {
        println!("Instruction: SUB {:?}", inst);
    }

    fn disassemble_SW(inst: Instruction<PC, IMM>) {
        println!("Instruction: SW {:?}", inst);
    }

    fn disassemble_XOR(inst: Instruction<PC, IMM>) {
        println!("Instruction: XOR {:?}", inst);
    }

    fn disassemble_XORI(inst: Instruction<PC, IMM>) {
        println!("Instruction: XORI {:?}", inst);
    }
}
