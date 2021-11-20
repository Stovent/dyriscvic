use crate::common::decoder::get_instruction_length;
use crate::common::isa::*;
use crate::public::*;
use crate::rvi::*;

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
            // self.x[self.inst.rd as usize] = self.inst.pc as i64 + self.inst.imm as i64;
        }
    }

    pub(crate) fn BEQ(&mut self) {
        if self.x[self.inst.rs1 as usize] == self.x[self.inst.rs2 as usize] {
            // let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
            // if self.is_misaligned(pc) {
            //     self.eei.trap(Traps::InstructionAddressMisaligned);
            // } else {
            //     self.pc = pc;
            // }
        }
    }

    pub(crate) fn BGE(&mut self) {
        if self.x[self.inst.rs1 as usize] >= self.x[self.inst.rs2 as usize] {
            // let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
            // if self.is_misaligned(pc) {
            //     self.eei.trap(Traps::InstructionAddressMisaligned);
            // } else {
            //     self.pc = pc;
            // }
        }
    }

    pub(crate) fn BGEU(&mut self) {
        // if (self.x[self.inst.rs1 as usize].as_u()) >= (self.x[self.inst.rs2 as usize].as_u()) {
        //     let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
        //     if self.is_misaligned(pc) {
        //         self.eei.trap(Traps::InstructionAddressMisaligned);
        //     } else {
        //         self.pc = pc;
        //     }
        // }
    }

    pub(crate) fn BLT(&mut self) {
        if self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize] {
            // let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
            // if self.is_misaligned(pc) {
            //     self.eei.trap(Traps::InstructionAddressMisaligned);
            // } else {
            //     self.pc = pc;
            // }
        }
    }

    pub(crate) fn BLTU(&mut self) {
        // if (self.x[self.inst.rs1 as usize].as_u()) < (self.x[self.inst.rs2 as usize].as_u()) {
        //     let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
        //     if self.is_misaligned(pc) {
        //         self.eei.trap(Traps::InstructionAddressMisaligned);
        //     } else {
        //         self.pc = pc;
        //     }
        // }
    }

    pub(crate) fn BNE(&mut self) {
        if self.x[self.inst.rs1 as usize] != self.x[self.inst.rs2 as usize] {
            // let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
            // if self.is_misaligned(pc) {
            //     self.eei.trap(Traps::InstructionAddressMisaligned);
            // } else {
            //     self.pc = pc;
            // }
        }
    }

    pub(crate) fn EBREAK(&mut self) {
        self.eei.trap(Trap::Breakpoint);
    }

    pub(crate) fn ECALL(&mut self) {
        self.eei.trap(Trap::SystemCall);
    }

    pub(crate) fn FENCE(&mut self) {
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
        // let pc = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u() & 0xFFFF_FFFEu32.into();
        // if self.is_misaligned(pc) {
        //     self.eei.trap(Traps::InstructionAddressMisaligned);
        // } else {
        //     self.pc = pc;
        //     if self.inst.rd != 0 {
        //         self.x[self.inst.rd as usize] = self.inst.pc.as_s() + 4.into();
        //     }
        // }
    }

    pub(crate) fn LB(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Trap::IllegalInstruction);
        } else {
            // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
            // self.x[self.inst.rd as usize] = (self.eei.get_8(addr) as i8).into();
        }
    }

    pub(crate) fn LBU(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Trap::IllegalInstruction);
        } else {
            // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
            // self.x[self.inst.rd as usize] = self.eei.get_8(addr).into();
        }
    }

    pub(crate) fn LH(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Trap::IllegalInstruction);
        } else {
            // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
            // self.x[self.inst.rd as usize] = (self.eei.get_16(addr) as i16).into();
        }
    }

    pub(crate) fn LHU(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Trap::IllegalInstruction);
        } else {
            // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
            // self.x[self.inst.rd as usize] = self.eei.get_16(addr).into();
        }
    }

    pub(crate) fn LUI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.imm as i64;
        }
    }

    pub(crate) fn LW(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Trap::IllegalInstruction);
        } else {
            // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
            // self.x[self.inst.rd as usize] = (self.eei.get_32(addr) as i32).into();
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
        // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
        // self.eei.set_8(addr, self.x[self.inst.rs2 as usize].as_u8());
    }

    pub(crate) fn SH(&mut self) {
        // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
        // self.eei.set_16(addr, self.x[self.inst.rs2 as usize].as_u16());
    }

    pub(crate) fn SLL(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.x[self.inst.rs2 as usize] & 0x3F.into());
        }
    }

    pub(crate) fn SLLI(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.inst.imm & 0x3F.into());
        }
    }

    pub(crate) fn SLT(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize]).into();
        }
    }

    pub(crate) fn SLTI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.inst.imm as i64).into();
        }
    }

    pub(crate) fn SLTIU(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() < (self.inst.imm as i64).as_u()).into();
        }
    }

    pub(crate) fn SLTU(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() < self.x[self.inst.rs2 as usize].as_u()).into();
        }
    }

    pub(crate) fn SRA(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.x[self.inst.rs2 as usize] & 0x3F.into());
        }
    }

    pub(crate) fn SRAI(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.inst.imm & 0x3Fi32.into());
        }
    }

    pub(crate) fn SRL(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() >> (self.x[self.inst.rs2 as usize].as_u() & 0x3Fu32.into())).as_s();
        }
    }

    pub(crate) fn SRLI(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() >> (self.inst.imm.as_u() & 0x3Fu32.into())).as_s();
        }
    }

    pub(crate) fn SUB(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] - self.x[self.inst.rs2 as usize];
        }
    }

    pub(crate) fn SW(&mut self) {
        // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
        // self.eei.set_32(addr, self.x[self.inst.rs2 as usize].as_u32());
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
    pub(crate) fn ADDIW(&mut self) {}

    pub(crate) fn ADDW(&mut self) {}

    pub(crate) fn LD(&mut self) {}

    pub(crate) fn LWU(&mut self) {}

    pub(crate) fn SD(&mut self) {}

    pub(crate) fn SLLIW(&mut self) {}

    pub(crate) fn SLLW(&mut self) {}

    pub(crate) fn SRAIW(&mut self) {}

    pub(crate) fn SRAW(&mut self) {}

    pub(crate) fn SRLIW(&mut self) {}

    pub(crate) fn SRLW(&mut self) {}

    pub(crate) fn SUBW(&mut self) {}
}
