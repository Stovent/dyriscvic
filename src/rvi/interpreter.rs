// use crate::common::decoder::get_instruction_length;
use crate::common::isa::*;
use crate::public::*;
use crate::rvi::*;

const I32_SHIFT_MASK: i32 = 0x1F;
const I64_SHIFT_MASK: i64 = 0x3F;

macro_rules! impl_rvi {
    ($name:ident) => {
        impl<EEI: ExecutionEnvironmentInterface> $name<EEI> {
            /// Executes a single intruction on the hart.
            pub fn single_step(&mut self) {
                let pc = self.pc;
                self.pc += 4;
                let opcode = self.eei.get_opcode_32(pc as u64); // TODO: instruction-address-misaligned
                // let inst_size = get_instruction_length(opcode as u16);
                // match inst_size {
                //     4 => {
                        let isa = Isa::from_opcode_32(opcode);
                        let entry = &self.isa[isa as usize];
                        self.inst = (entry.decoder)(isa, pc, opcode);

                        #[cfg(debug_assertions)]
                        println!("{:X}: {}", pc, (entry.disassemble)(self.inst, self.config.abi_name));

                        (entry.execute)(self);
                //     },
                //     _ => println!("Unknown opcode {:#X} at {:#X}", opcode, pc),
                // };
            }
        }
    };
}

impl_rvi!(RV32I);
impl_rvi!(RV64I);

macro_rules! impl_rv32i {
    ($name:ident, $regtype:ty, $pctype:ty, $shift_mask:expr) => {
        impl<EEI: ExecutionEnvironmentInterface> $name<EEI> {
            pub(crate) fn UNKNOWN(&mut self) {
                self.eei.trap(Trap::IllegalInstruction);
            }

            pub(crate) fn ADD(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] + self.x[self.inst.rs2 as usize];
                }
            }

            pub(crate) fn ADDI(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] + self.inst.imm as $regtype;
                }
            }

            pub(crate) fn AND(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] & self.x[self.inst.rs2 as usize];
                }
            }

            pub(crate) fn ANDI(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] & self.inst.imm as $regtype;
                }
            }

            pub(crate) fn AUIPC(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.inst.pc as $regtype + self.inst.imm as $regtype;
                }
            }

            pub(crate) fn BEQ(&mut self) {
                if self.x[self.inst.rs1 as usize] == self.x[self.inst.rs2 as usize] {
                    let pc = self.inst.pc + self.inst.imm as $pctype;
                    if self.is_misaligned(pc) {
                        self.eei.trap(Trap::InstructionAddressMisaligned);
                    } else {
                        self.pc = pc;
                    }
                }
            }

            pub(crate) fn BGE(&mut self) {
                if self.x[self.inst.rs1 as usize] >= self.x[self.inst.rs2 as usize] {
                    let pc = self.inst.pc + self.inst.imm as $pctype;
                    if self.is_misaligned(pc) {
                        self.eei.trap(Trap::InstructionAddressMisaligned);
                    } else {
                        self.pc = pc;
                    }
                }
            }

            pub(crate) fn BGEU(&mut self) {
                if (self.x[self.inst.rs1 as usize] as $pctype) >= (self.x[self.inst.rs2 as usize] as $pctype) {
                    let pc = self.inst.pc + self.inst.imm as $pctype;
                    if self.is_misaligned(pc) {
                        self.eei.trap(Trap::InstructionAddressMisaligned);
                    } else {
                        self.pc = pc;
                    }
                }
            }

            pub(crate) fn BLT(&mut self) {
                if self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize] {
                    let pc = self.inst.pc + self.inst.imm as $pctype;
                    if self.is_misaligned(pc) {
                        self.eei.trap(Trap::InstructionAddressMisaligned);
                    } else {
                        self.pc = pc;
                    }
                }
            }

            pub(crate) fn BLTU(&mut self) {
                if (self.x[self.inst.rs1 as usize] as $pctype) < (self.x[self.inst.rs2 as usize] as $pctype) {
                    let pc = self.inst.pc + self.inst.imm as $pctype;
                    if self.is_misaligned(pc) {
                        self.eei.trap(Trap::InstructionAddressMisaligned);
                    } else {
                        self.pc = pc;
                    }
                }
            }

            pub(crate) fn BNE(&mut self) {
                if self.x[self.inst.rs1 as usize] != self.x[self.inst.rs2 as usize] {
                    let pc = self.inst.pc + self.inst.imm as $pctype;
                    if self.is_misaligned(pc) {
                        self.eei.trap(Trap::InstructionAddressMisaligned);
                    } else {
                        self.pc = pc;
                    }
                }
            }

            pub(crate) fn EBREAK(&mut self) {
                self.eei.trap(Trap::Breakpoint);
            }

            pub(crate) fn ECALL(&mut self) {
                self.eei.trap(Trap::SystemCall);
            }

            pub(crate) fn FENCE(&mut self) {
                // TODO
            }

            pub(crate) fn JAL(&mut self) {
                let pc = self.inst.pc + self.inst.imm as $pctype;
                if self.is_misaligned(pc) {
                    self.eei.trap(Trap::InstructionAddressMisaligned);
                } else {
                    self.pc = pc;
                    if self.inst.rd != 0 {
                        self.x[self.inst.rd as usize] = self.inst.pc as $regtype + 4;
                    }
                }
            }

            pub(crate) fn JALR(&mut self) {
                let pc = self.x[self.inst.rs1 as usize] as $pctype + self.inst.imm as $pctype & !1;
                if self.is_misaligned(pc) {
                    self.eei.trap(Trap::InstructionAddressMisaligned);
                } else {
                    self.pc = pc;
                    if self.inst.rd != 0 {
                        self.x[self.inst.rd as usize] = self.inst.pc as $regtype + 4;
                    }
                }
            }

            pub(crate) fn LB(&mut self) {
                // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
                let addr = self.x[self.inst.rs1 as usize] as $pctype + self.inst.imm as $pctype;
                let data = self.eei.get_8(addr as u64) as i8 as $regtype;

                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = data;
                }
            }

            pub(crate) fn LBU(&mut self) {
                // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
                let addr = self.x[self.inst.rs1 as usize] as $pctype + self.inst.imm as $pctype;
                let data = self.eei.get_8(addr as u64) as $regtype;

                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = data;
                }
            }

            pub(crate) fn LH(&mut self) {
                // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
                let addr = self.x[self.inst.rs1 as usize] as $pctype + self.inst.imm as $pctype;
                let data = self.eei.get_16(addr as u64) as i16 as $regtype;

                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = data;
                }
            }

            pub(crate) fn LHU(&mut self) {
                // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
                let addr = self.x[self.inst.rs1 as usize] as $pctype + self.inst.imm as $pctype;
                let data = self.eei.get_16(addr as u64) as $regtype;

                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = data;
                }
            }

            pub(crate) fn LUI(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.inst.imm as $regtype;
                }
            }

            pub(crate) fn LW(&mut self) {
                // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
                let addr = self.x[self.inst.rs1 as usize] as $pctype + self.inst.imm as $pctype;
                let data = self.eei.get_32(addr as u64) as i32 as $regtype;

                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = data;
                }
            }

            pub(crate) fn OR(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] | self.x[self.inst.rs2 as usize];
                }
            }

            pub(crate) fn ORI(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] | self.inst.imm as $regtype;
                }
            }

            pub(crate) fn SB(&mut self) {
                // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
                let addr = self.x[self.inst.rs1 as usize] as $pctype + self.inst.imm as $pctype;
                self.eei.set_8(addr as u64, self.x[self.inst.rs2 as usize] as u8);
            }

            pub(crate) fn SH(&mut self) {
                // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
                let addr = self.x[self.inst.rs1 as usize] as $pctype + self.inst.imm as $pctype;
                self.eei.set_16(addr as u64, self.x[self.inst.rs2 as usize] as u16);
            }

            pub(crate) fn SLL(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.x[self.inst.rs2 as usize] & $shift_mask);
                }
            }

            pub(crate) fn SLLI(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.inst.imm as $regtype & $shift_mask);
                }
            }

            pub(crate) fn SLT(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize]) as $regtype;
                }
            }

            pub(crate) fn SLTI(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.inst.imm as $regtype) as $regtype;
                }
            }

            pub(crate) fn SLTIU(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as $pctype) < self.inst.imm as $pctype) as $regtype;
                }
            }

            pub(crate) fn SLTU(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as $pctype) < self.x[self.inst.rs2 as usize] as $pctype) as $regtype;
                }
            }

            pub(crate) fn SRA(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.x[self.inst.rs2 as usize] & $shift_mask);
                }
            }

            pub(crate) fn SRAI(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.inst.imm as $regtype & $shift_mask);
                }
            }

            pub(crate) fn SRL(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] as $pctype >> (self.x[self.inst.rs2 as usize] & $shift_mask) as $pctype) as $regtype;
                }
            }

            pub(crate) fn SRLI(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] as $pctype >> (self.inst.imm as $pctype & $shift_mask as $pctype)) as $regtype;
                }
            }

            pub(crate) fn SUB(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] - self.x[self.inst.rs2 as usize];
                }
            }

            pub(crate) fn SW(&mut self) {
                // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
                let addr = self.x[self.inst.rs1 as usize] as $pctype + self.inst.imm as $pctype;
                self.eei.set_32(addr as u64, self.x[self.inst.rs2 as usize] as u32);
            }

            pub(crate) fn XOR(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] ^ self.x[self.inst.rs2 as usize];
                }
            }

            pub(crate) fn XORI(&mut self) {
                if self.inst.rd != 0 {
                    self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] ^ self.inst.imm as $regtype;
                }
            }
        }
    };
}

impl<EEI: ExecutionEnvironmentInterface> RV64I<EEI> {
    pub(crate) fn ADDIW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] as i32 as i64 + self.inst.imm as i64;
        }
    }

    pub(crate) fn ADDW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] as i32 as i64 + self.x[self.inst.rs2 as usize] as i32 as i64;
        }
    }

    pub(crate) fn LD(&mut self) {
        // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
        let addr = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64;
        let data = self.eei.get_64(addr) as i64;

        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = data;
        }
    }

    pub(crate) fn LWU(&mut self) {
        // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
        let addr = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64;
        let data = self.eei.get_32(addr) as i64;

        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = data;
        }
    }

    pub(crate) fn SD(&mut self) {
        // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
        let addr = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64;
        self.eei.set_64(addr, self.x[self.inst.rs2 as usize] as u64);
    }

    pub(crate) fn SLLIW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as i32) << (self.inst.imm & I32_SHIFT_MASK)) as i64;
        }
    }

    pub(crate) fn SLLW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as i32) << (self.x[self.inst.rs2 as usize] as i32 & I32_SHIFT_MASK)) as i64;
        }
    }

    pub(crate) fn SRAIW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] as i32 >> (self.inst.imm & I32_SHIFT_MASK)) as i64;
        }
    }

    pub(crate) fn SRAW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] as i32 >> (self.x[self.inst.rs2 as usize] as i32 & I32_SHIFT_MASK)) as i64;
        }
    }

    pub(crate) fn SRLIW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] as u32 >> (self.inst.imm as u32 & I32_SHIFT_MASK as u32)) as i64;
        }
    }

    pub(crate) fn SRLW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] as u32 >> (self.x[self.inst.rs2 as usize] as u32 & I32_SHIFT_MASK as u32)) as i64;
        }
    }

    pub(crate) fn SUBW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] as i32 as i64 - self.x[self.inst.rs2 as usize] as i32 as i64;
        }
    }
}

impl_rv32i!(RV32I, i32, u32, I32_SHIFT_MASK);
impl_rv32i!(RV64I, i64, u64, I64_SHIFT_MASK);
