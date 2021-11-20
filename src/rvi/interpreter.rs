use crate::common::decoder::get_instruction_length;
use crate::common::isa::*;
use crate::public::*;
use crate::rvi::*;

const I64_SHIFT_MASK: i64 = 0x3F;
const I32_SHIFT_MASK: i32 = 0x1F;

impl<EEI: ExecutionEnvironmentInterface> RV64I<EEI> {
    /// Executes a single intruction on the hart.
    pub fn single_step(&mut self) {
        let pc = self.pc;
        self.pc += 4;
        let opcode = self.eei.get_opcode_32(pc); // TODO: instruction-address-misaligned
        let inst_size = get_instruction_length(opcode as u16);
        match inst_size {
            4 => {
                let isa = Isa::from_opcode_32(opcode);
                let entry = &self.isa[isa as usize];
                self.inst = (entry.decoder)(isa, pc, opcode);

                #[cfg(debug_assertions)]
                println!("Instruction: {}", (entry.disassemble)(self.inst, self.config.abi_name));

                (entry.execute)(self);
            },
            _ => println!("Unknown opcode {:#X} at {:#X}", opcode, pc),
        };
    }
}

impl<EEI: ExecutionEnvironmentInterface> RV64I<EEI> {
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
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] + self.inst.imm as i64;
        }
    }

    pub(crate) fn AND(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] & self.x[self.inst.rs2 as usize];
        }
    }

    pub(crate) fn ANDI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] & self.inst.imm as i64;
        }
    }

    pub(crate) fn AUIPC(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.pc as i64 + self.inst.imm as i64;
        }
    }

    pub(crate) fn BEQ(&mut self) {
        if self.x[self.inst.rs1 as usize] == self.x[self.inst.rs2 as usize] {
            let pc = self.inst.pc + self.inst.imm as u64;
            if self.is_misaligned(pc) {
                self.eei.trap(Trap::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    pub(crate) fn BGE(&mut self) {
        if self.x[self.inst.rs1 as usize] >= self.x[self.inst.rs2 as usize] {
            let pc = self.inst.pc + self.inst.imm as u64;
            if self.is_misaligned(pc) {
                self.eei.trap(Trap::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    pub(crate) fn BGEU(&mut self) {
        if (self.x[self.inst.rs1 as usize] as u64) >= (self.x[self.inst.rs2 as usize] as u64) {
            let pc = self.inst.pc + self.inst.imm as u64;
            if self.is_misaligned(pc) {
                self.eei.trap(Trap::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    pub(crate) fn BLT(&mut self) {
        if self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize] {
            let pc = self.inst.pc + self.inst.imm as u64;
            if self.is_misaligned(pc) {
                self.eei.trap(Trap::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    pub(crate) fn BLTU(&mut self) {
        if (self.x[self.inst.rs1 as usize] as u64) < (self.x[self.inst.rs2 as usize] as u64) {
            let pc = self.inst.pc + self.inst.imm as u64;
            if self.is_misaligned(pc) {
                self.eei.trap(Trap::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    pub(crate) fn BNE(&mut self) {
        if self.x[self.inst.rs1 as usize] != self.x[self.inst.rs2 as usize] {
            let pc = self.inst.pc + self.inst.imm as u64;
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
        let pc = self.inst.pc + self.inst.imm as u64;
        if self.is_misaligned(pc) {
            self.eei.trap(Trap::InstructionAddressMisaligned);
        } else {
            self.pc = pc;
            if self.inst.rd != 0 {
                self.x[self.inst.rd as usize] = self.inst.pc as i64 + 4;
            }
        }
    }

    pub(crate) fn JALR(&mut self) {
        let pc = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64 & !1;
        if self.is_misaligned(pc) {
            self.eei.trap(Trap::InstructionAddressMisaligned);
        } else {
            self.pc = pc;
            if self.inst.rd != 0 {
                self.x[self.inst.rd as usize] = self.inst.pc as i64 + 4;
            }
        }
    }

    pub(crate) fn LB(&mut self) {
        // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
        let addr = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64;
        let data = self.eei.get_8(addr) as i8 as i64;

        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = data;
        }
    }

    pub(crate) fn LBU(&mut self) {
        // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
        let addr = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64;
        let data = self.eei.get_8(addr) as i64;

        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = data;
        }
    }

    pub(crate) fn LH(&mut self) {
        // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
        let addr = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64;
        let data = self.eei.get_16(addr) as i16 as i64;

        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = data;
        }
    }

    pub(crate) fn LHU(&mut self) {
        // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
        let addr = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64;
        let data = self.eei.get_16(addr) as i64;

        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = data;
        }
    }

    pub(crate) fn LUI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.imm as i64;
        }
    }

    pub(crate) fn LW(&mut self) {
        // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
        let addr = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64;
        let data = self.eei.get_32(addr) as i32 as i64;

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
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] | self.inst.imm as i64;
        }
    }

    pub(crate) fn SB(&mut self) {
        // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
        let addr = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64;
        self.eei.set_8(addr, self.x[self.inst.rs2 as usize] as u8);
    }

    pub(crate) fn SH(&mut self) {
        // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
        let addr = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64;
        self.eei.set_16(addr, self.x[self.inst.rs2 as usize] as u16);
    }

    pub(crate) fn SLL(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.x[self.inst.rs2 as usize] & I64_SHIFT_MASK);
        }
    }

    pub(crate) fn SLLI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.inst.imm as i64 & I64_SHIFT_MASK);
        }
    }

    pub(crate) fn SLT(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize]) as i64;
        }
    }

    pub(crate) fn SLTI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.inst.imm as i64) as i64;
        }
    }

    pub(crate) fn SLTIU(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as u64) < self.inst.imm as i64 as u64) as i64;
        }
    }

    pub(crate) fn SLTU(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as u64) < self.x[self.inst.rs2 as usize] as u64) as i64;
        }
    }

    pub(crate) fn SRA(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.x[self.inst.rs2 as usize] & I64_SHIFT_MASK);
        }
    }

    pub(crate) fn SRAI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.inst.imm as i64 & I64_SHIFT_MASK);
        }
    }

    pub(crate) fn SRL(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] as u64 >> (self.x[self.inst.rs2 as usize] & I64_SHIFT_MASK) as u64) as i64;
        }
    }

    pub(crate) fn SRLI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] as u64 >> (self.inst.imm as u64 & I64_SHIFT_MASK as u64)) as i64;
        }
    }

    pub(crate) fn SUB(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] - self.x[self.inst.rs2 as usize];
        }
    }

    pub(crate) fn SW(&mut self) {
        // TODO: Loads and stores where the effective address is not naturally aligned to the referenced datatype have behavior dependent on the EEI.
        let addr = self.x[self.inst.rs1 as usize] as u64 + self.inst.imm as u64;
        self.eei.set_32(addr, self.x[self.inst.rs2 as usize] as u32);
    }

    pub(crate) fn XOR(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] ^ self.x[self.inst.rs2 as usize];
        }
    }

    pub(crate) fn XORI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] ^ self.inst.imm as i64;
        }
    }

    // I64
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
