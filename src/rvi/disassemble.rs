use crate::common::{isa::*, types::*};
use crate::rvi::*;

impl<U: Unsigned<S>, S: Signed<U>, EEI: ExecutionEnvironmentInterface<U>, const N: usize> DisassembleI32<U, S, EEI, N> for RVI<U, S, EEI, N> {
    fn load_disassemble_i32(&mut self) {
        self.disassemble[ISA::ADD as usize..=ISA::XORI as usize].copy_from_slice(&Self::DISASSEMBLE_I32);
    }

    fn disassemble_UNKNOWN(inst: Instruction<U, S>, _: bool) -> String {
        format!("Unknown instruction {:?}", inst)
    }

    fn disassemble_ADD(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("add {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_ADDI(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("addi {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_AND(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("and {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_ANDI(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("andi {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_AUIPC(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("auipc {}, {}", get_x_register_name(inst.rd, abi_name), inst.imm)
    }

    fn disassemble_BEQ(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("beq {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    fn disassemble_BGE(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("bge {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    fn disassemble_BGEU(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("bgeu {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    fn disassemble_BLT(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("blt {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    fn disassemble_BLTU(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("bltu {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    fn disassemble_BNE(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("bne {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    fn disassemble_EBREAK(_: Instruction<U, S>, _: bool) -> String {
        format!("ebreak")
    }

    fn disassemble_ECALL(_: Instruction<U, S>, _: bool) -> String {
        format!("ecall")
    }

    fn disassemble_FENCE(inst: Instruction<U, S>, _: bool) -> String {
        format!("fence {:?}", inst)
    }

    fn disassemble_JAL(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("jal {}, {}", get_x_register_name(inst.rd, abi_name), inst.imm)
    }

    fn disassemble_JALR(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("jalr {}, {}({})", get_x_register_name(inst.rd, abi_name), inst.imm, get_x_register_name(inst.rs1, abi_name))
    }

    fn disassemble_LB(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("lb {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_LBU(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("lbu {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_LH(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("lh {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_LHU(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("lhu {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_LUI(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("lui {}, {}", get_x_register_name(inst.rd, abi_name), inst.imm)
    }

    fn disassemble_LW(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("lw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_OR(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("or {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_ORI(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("ori {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_SB(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("sb {}, {}, {}", get_x_register_name(inst.rs2, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_SH(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("sh {}, {}, {}", get_x_register_name(inst.rs2, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_SLL(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("sll {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_SLLI(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("slli {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F.into())
    }

    fn disassemble_SLT(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("slt {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_SLTI(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("slti {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_SLTIU(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("sltiu {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_SLTU(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("sltu {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_SRA(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("sra {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_SRAI(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("srai {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F.into())
    }

    fn disassemble_SRL(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("srl {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_SRLI(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("srli {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F.into())
    }

    fn disassemble_SUB(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("sub {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_SW(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("sw {}, {}, {}", get_x_register_name(inst.rs2, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_XOR(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("xor {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_XORI(inst: Instruction<U, S>, abi_name: bool) -> String {
        format!("xori {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }
}

impl<EEI: ExecutionEnvironmentInterface<u64>> DisassembleI64<u64, i64> for RV64I<EEI> {
    fn load_disassemble_i64(&mut self) {
        self.disassemble[ISA::ADDIW as usize..=ISA::SUBW as usize].copy_from_slice(&Self::DISASSEMBLE_I64);
    }

    fn disassemble_ADDIW(inst: Instruction64, abi_name: bool) -> String {
        format!("addiw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_ADDW(inst: Instruction64, abi_name: bool) -> String {
        format!("addw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_LD(inst: Instruction64, abi_name: bool) -> String {
        format!("ld {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_LWU(inst: Instruction64, abi_name: bool) -> String {
        format!("lwu {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_SD(inst: Instruction64, abi_name: bool) -> String {
        format!("sd {}, {}, {}", get_x_register_name(inst.rs2, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_SLLIW(inst: Instruction64, abi_name: bool) -> String {
        format!("slliw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_SLLW(inst: Instruction64, abi_name: bool) -> String {
        format!("sllw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_SRAIW(inst: Instruction64, abi_name: bool) -> String {
        format!("sraiw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_SRAW(inst: Instruction64, abi_name: bool) -> String {
        format!("sraw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_SRLIW(inst: Instruction64, abi_name: bool) -> String {
        format!("srliw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    fn disassemble_SRLW(inst: Instruction64, abi_name: bool) -> String {
        format!("srlw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    fn disassemble_SUBW(inst: Instruction64, abi_name: bool) -> String {
        format!("subw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }
}
