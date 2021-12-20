use crate::rvi::*;

macro_rules! impl_disassembler {
    ($name:ident, $inst:ty) => {
        impl<EEI: ExecutionEnvironmentInterface> $name<EEI> {
            pub(crate) fn disassemble_UNKNOWN(inst: $inst, _: bool) -> String {
                format!("Unknown instruction {:?}", inst)
            }

            pub(crate) fn disassemble_ADD(inst: $inst, abi_name: bool) -> String {
                format!("add {}", inst.disassemble_type_r(abi_name))
            }

            pub(crate) fn disassemble_ADDI(inst: $inst, abi_name: bool) -> String {
                format!("addi {}", inst.disassemble_type_i(abi_name))
            }

            pub(crate) fn disassemble_AND(inst: $inst, abi_name: bool) -> String {
                format!("and {}", inst.disassemble_type_r(abi_name))
            }

            pub(crate) fn disassemble_ANDI(inst: $inst, abi_name: bool) -> String {
                format!("andi {}", inst.disassemble_type_i(abi_name))
            }

            pub(crate) fn disassemble_AUIPC(inst: $inst, abi_name: bool) -> String {
                format!("auipc {}", inst.disassemble_type_u_j(abi_name))
            }

            pub(crate) fn disassemble_BEQ(inst: $inst, abi_name: bool) -> String {
                format!("beq {}", inst.disassemble_type_b(abi_name))
            }

            pub(crate) fn disassemble_BGE(inst: $inst, abi_name: bool) -> String {
                format!("bge {}", inst.disassemble_type_b(abi_name))
            }

            pub(crate) fn disassemble_BGEU(inst: $inst, abi_name: bool) -> String {
                format!("bgeu {}", inst.disassemble_type_b(abi_name))
            }

            pub(crate) fn disassemble_BLT(inst: $inst, abi_name: bool) -> String {
                format!("blt {}", inst.disassemble_type_b(abi_name))
            }

            pub(crate) fn disassemble_BLTU(inst: $inst, abi_name: bool) -> String {
                format!("bltu {}", inst.disassemble_type_b(abi_name))
            }

            pub(crate) fn disassemble_BNE(inst: $inst, abi_name: bool) -> String {
                format!("bne {}", inst.disassemble_type_b(abi_name))
            }

            pub(crate) fn disassemble_EBREAK(_: $inst, _: bool) -> String {
                format!("ebreak")
            }

            pub(crate) fn disassemble_ECALL(_: $inst, _: bool) -> String {
                format!("ecall")
            }

            pub(crate) fn disassemble_FENCE(inst: $inst, _: bool) -> String {
                format!("fence {:?}", inst)
            }

            pub(crate) fn disassemble_JAL(inst: $inst, abi_name: bool) -> String {
                format!("jal {}", inst.disassemble_type_u_j(abi_name))
            }

            pub(crate) fn disassemble_JALR(inst: $inst, abi_name: bool) -> String {
                format!("jalr {}, {}({})", get_x_register_name(inst.rd, abi_name), inst.imm, get_x_register_name(inst.rs1, abi_name))
            }

            pub(crate) fn disassemble_LB(inst: $inst, abi_name: bool) -> String {
                format!("lb {}", inst.disassemble_type_i(abi_name))
            }

            pub(crate) fn disassemble_LBU(inst: $inst, abi_name: bool) -> String {
                format!("lbu {}", inst.disassemble_type_i(abi_name))
            }

            pub(crate) fn disassemble_LH(inst: $inst, abi_name: bool) -> String {
                format!("lh {}", inst.disassemble_type_i(abi_name))
            }

            pub(crate) fn disassemble_LHU(inst: $inst, abi_name: bool) -> String {
                format!("lhu {}", inst.disassemble_type_i(abi_name))
            }

            pub(crate) fn disassemble_LUI(inst: $inst, abi_name: bool) -> String {
                format!("lui {}", inst.disassemble_type_u_j(abi_name))
            }

            pub(crate) fn disassemble_LW(inst: $inst, abi_name: bool) -> String {
                format!("lw {}", inst.disassemble_type_i(abi_name))
            }

            pub(crate) fn disassemble_OR(inst: $inst, abi_name: bool) -> String {
                format!("or {}", inst.disassemble_type_r(abi_name))
            }

            pub(crate) fn disassemble_ORI(inst: $inst, abi_name: bool) -> String {
                format!("ori {}", inst.disassemble_type_i(abi_name))
            }

            pub(crate) fn disassemble_SB(inst: $inst, abi_name: bool) -> String {
                format!("sb {}", inst.disassemble_type_s(abi_name))
            }

            pub(crate) fn disassemble_SH(inst: $inst, abi_name: bool) -> String {
                format!("sh {}", inst.disassemble_type_s(abi_name))
            }

            pub(crate) fn disassemble_SLL(inst: $inst, abi_name: bool) -> String {
                format!("sll {}", inst.disassemble_type_r(abi_name))
            }

            pub(crate) fn disassemble_SLLI(inst: $inst, abi_name: bool) -> String {
                format!("slli {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x3F)
            }

            pub(crate) fn disassemble_SLT(inst: $inst, abi_name: bool) -> String {
                format!("slt {}", inst.disassemble_type_r(abi_name))
            }

            pub(crate) fn disassemble_SLTI(inst: $inst, abi_name: bool) -> String {
                format!("slti {}", inst.disassemble_type_i(abi_name))
            }

            pub(crate) fn disassemble_SLTIU(inst: $inst, abi_name: bool) -> String {
                format!("sltiu {}", inst.disassemble_type_i(abi_name))
            }

            pub(crate) fn disassemble_SLTU(inst: $inst, abi_name: bool) -> String {
                format!("sltu {}", inst.disassemble_type_r(abi_name))
            }

            pub(crate) fn disassemble_SRA(inst: $inst, abi_name: bool) -> String {
                format!("sra {}", inst.disassemble_type_r(abi_name))
            }

            pub(crate) fn disassemble_SRAI(inst: $inst, abi_name: bool) -> String {
                format!("srai {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x3F)
            }

            pub(crate) fn disassemble_SRL(inst: $inst, abi_name: bool) -> String {
                format!("srl {}", inst.disassemble_type_r(abi_name))
            }

            pub(crate) fn disassemble_SRLI(inst: $inst, abi_name: bool) -> String {
                format!("srli {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x3F)
            }

            pub(crate) fn disassemble_SUB(inst: $inst, abi_name: bool) -> String {
                format!("sub {}", inst.disassemble_type_r(abi_name))
            }

            pub(crate) fn disassemble_SW(inst: $inst, abi_name: bool) -> String {
                format!("sw {}", inst.disassemble_type_s(abi_name))
            }

            pub(crate) fn disassemble_XOR(inst: $inst, abi_name: bool) -> String {
                format!("xor {}", inst.disassemble_type_r(abi_name))
            }

            pub(crate) fn disassemble_XORI(inst: $inst, abi_name: bool) -> String {
                format!("xori {}", inst.disassemble_type_i(abi_name))
            }
        }
    };
}

impl<EEI: ExecutionEnvironmentInterface> RV64I<EEI> {
    // I64
    pub(crate) fn disassemble_ADDIW(inst: Instruction64, abi_name: bool) -> String {
        format!("addiw {}", inst.disassemble_type_i(abi_name))
    }

    pub(crate) fn disassemble_ADDW(inst: Instruction64, abi_name: bool) -> String {
        format!("addw {}", inst.disassemble_type_r(abi_name))
    }

    pub(crate) fn disassemble_LD(inst: Instruction64, abi_name: bool) -> String {
        format!("ld {}", inst.disassemble_type_i(abi_name))
    }

    pub(crate) fn disassemble_LWU(inst: Instruction64, abi_name: bool) -> String {
        format!("lwu {}", inst.disassemble_type_i(abi_name))
    }

    pub(crate) fn disassemble_SD(inst: Instruction64, abi_name: bool) -> String {
        format!("sd {}", inst.disassemble_type_s(abi_name))
    }

    pub(crate) fn disassemble_SLLIW(inst: Instruction64, abi_name: bool) -> String {
        format!("slliw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F)
    }

    pub(crate) fn disassemble_SLLW(inst: Instruction64, abi_name: bool) -> String {
        format!("sllw {}", inst.disassemble_type_r(abi_name))
    }

    pub(crate) fn disassemble_SRAIW(inst: Instruction64, abi_name: bool) -> String {
        format!("sraiw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F)
    }

    pub(crate) fn disassemble_SRAW(inst: Instruction64, abi_name: bool) -> String {
        format!("sraw {}", inst.disassemble_type_r(abi_name))
    }

    pub(crate) fn disassemble_SRLIW(inst: Instruction64, abi_name: bool) -> String {
        format!("srliw {}, {}, {}", get_x_register_name(inst.rd, abi_name), get_x_register_name(inst.rs1, abi_name), inst.imm & 0x1F)
    }

    pub(crate) fn disassemble_SRLW(inst: Instruction64, abi_name: bool) -> String {
        format!("srlw {}", inst.disassemble_type_r(abi_name))
    }

    pub(crate) fn disassemble_SUBW(inst: Instruction64, abi_name: bool) -> String {
        format!("subw {}", inst.disassemble_type_r(abi_name))
    }
}

impl_disassembler!(RV32I, Instruction32);
impl_disassembler!(RV64I, Instruction64);
