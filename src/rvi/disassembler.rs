use crate::rvi::*;

impl<EEI: ExecutionEnvironmentInterface> RV64I<EEI> {
    pub(crate) fn disassemble_UNKNOWN(inst: Instruction, _: bool) -> String {
        format!("Unknown instruction {:?}", inst)
    }

    pub(crate) fn disassemble_ADD(inst: Instruction, abi_name: bool) -> String {
        format!("add {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_ADDI(inst: Instruction, abi_name: bool) -> String {
        format!("addi {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_AND(inst: Instruction, abi_name: bool) -> String {
        format!("and {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_ANDI(inst: Instruction, abi_name: bool) -> String {
        format!("andi {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_AUIPC(inst: Instruction, abi_name: bool) -> String {
        format!("auipc {}, {}", get_x_register_name(inst.rd, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_BEQ(inst: Instruction, abi_name: bool) -> String {
        format!("beq {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_BGE(inst: Instruction, abi_name: bool) -> String {
        format!("bge {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_BGEU(inst: Instruction, abi_name: bool) -> String {
        format!("bgeu {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_BLT(inst: Instruction, abi_name: bool) -> String {
        format!("blt {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_BLTU(inst: Instruction, abi_name: bool) -> String {
        format!("bltu {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_BNE(inst: Instruction, abi_name: bool) -> String {
        format!("bne {}, {}, {}", get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_EBREAK(_: Instruction, _: bool) -> String {
        format!("ebreak")
    }

    pub(crate) fn disassemble_ECALL(_: Instruction, _: bool) -> String {
        format!("ecall")
    }

    pub(crate) fn disassemble_FENCE(inst: Instruction, _: bool) -> String {
        format!("fence {:?}", inst)
    }

    pub(crate) fn disassemble_JAL(inst: Instruction, abi_name: bool) -> String {
        format!("jal {}, {}", get_x_register_name(inst.rd, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_JALR(inst: Instruction, abi_name: bool) -> String {
        format!("jalr {}, {}({})", get_x_register_name(inst.rd, abi_name), inst.imm, get_x_register_name(inst.rs1, abi_name))
    }

    pub(crate) fn disassemble_LB(inst: Instruction, abi_name: bool) -> String {
        format!("lb {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_LBU(inst: Instruction, abi_name: bool) -> String {
        format!("lbu {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_LH(inst: Instruction, abi_name: bool) -> String {
        format!("lh {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_LHU(inst: Instruction, abi_name: bool) -> String {
        format!("lhu {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_LUI(inst: Instruction, abi_name: bool) -> String {
        format!("lui {}, {}", get_x_register_name(inst.rd, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_LW(inst: Instruction, abi_name: bool) -> String {
        format!("lw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_OR(inst: Instruction, abi_name: bool) -> String {
        format!("or {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_ORI(inst: Instruction, abi_name: bool) -> String {
        format!("ori {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_SB(inst: Instruction, abi_name: bool) -> String {
        format!("sb {}, {}, {}", get_x_register_name(inst.rs2, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_SH(inst: Instruction, abi_name: bool) -> String {
        format!("sh {}, {}, {}", get_x_register_name(inst.rs2, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_SLL(inst: Instruction, abi_name: bool) -> String {
        format!("sll {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_SLLI(inst: Instruction, abi_name: bool) -> String {
        format!("slli {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F)
    }

    pub(crate) fn disassemble_SLT(inst: Instruction, abi_name: bool) -> String {
        format!("slt {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_SLTI(inst: Instruction, abi_name: bool) -> String {
        format!("slti {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_SLTIU(inst: Instruction, abi_name: bool) -> String {
        format!("sltiu {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_SLTU(inst: Instruction, abi_name: bool) -> String {
        format!("sltu {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_SRA(inst: Instruction, abi_name: bool) -> String {
        format!("sra {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_SRAI(inst: Instruction, abi_name: bool) -> String {
        format!("srai {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F)
    }

    pub(crate) fn disassemble_SRL(inst: Instruction, abi_name: bool) -> String {
        format!("srl {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_SRLI(inst: Instruction, abi_name: bool) -> String {
        format!("srli {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F)
    }

    pub(crate) fn disassemble_SUB(inst: Instruction, abi_name: bool) -> String {
        format!("sub {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_SW(inst: Instruction, abi_name: bool) -> String {
        format!("sw {}, {}, {}", get_x_register_name(inst.rs2, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_XOR(inst: Instruction, abi_name: bool) -> String {
        format!("xor {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_XORI(inst: Instruction, abi_name: bool) -> String {
        format!("xori {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    // I64
    pub(crate) fn disassemble_ADDIW(inst: Instruction, abi_name: bool) -> String {
        format!("addiw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_ADDW(inst: Instruction, abi_name: bool) -> String {
        format!("addw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_LD(inst: Instruction, abi_name: bool) -> String {
        format!("ld {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_LWU(inst: Instruction, abi_name: bool) -> String {
        format!("lwu {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_SD(inst: Instruction, abi_name: bool) -> String {
        format!("sd {}, {}, {}", get_x_register_name(inst.rs2, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm)
    }

    pub(crate) fn disassemble_SLLIW(inst: Instruction, abi_name: bool) -> String {
        format!("slliw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F)
    }

    pub(crate) fn disassemble_SLLW(inst: Instruction, abi_name: bool) -> String {
        format!("sllw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_SRAIW(inst: Instruction, abi_name: bool) -> String {
        format!("sraiw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F)
    }

    pub(crate) fn disassemble_SRAW(inst: Instruction, abi_name: bool) -> String {
        format!("sraw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_SRLIW(inst: Instruction, abi_name: bool) -> String {
        format!("srliw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F)
    }

    pub(crate) fn disassemble_SRLW(inst: Instruction, abi_name: bool) -> String {
        format!("srlw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }

    pub(crate) fn disassemble_SUBW(inst: Instruction, abi_name: bool) -> String {
        format!("subw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), get_x_register_name(inst.rs2, abi_name))
    }
}
