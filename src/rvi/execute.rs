use crate::common::isa::*;
use crate::public::*;
use crate::rvi::*;

impl<U: Unsigned<S>, S: Signed<U>, EEI: ExecutionEnvironmentInterface<U>, const N: usize> I32<U, S, EEI, N> for RVI<U, S, EEI, N> {
    fn load_execute_i32(&mut self) {
        self.execute[ISA::ADD as usize..=ISA::XORI as usize].copy_from_slice(&RVI::<U, S, EEI, N>::EXECUTE_I32);
    }

    fn UNKNOWN(&mut self) {
        self.eei.trap(Traps::IllegalInstruction);
    }

    fn ADD(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] + self.x[self.inst.rs2 as usize];
        }
    }

    fn ADDI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] + self.inst.imm;
        }
    }

    fn AND(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] & self.x[self.inst.rs2 as usize];
        }
    }

    fn ANDI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] & self.inst.imm;
        }
    }

    fn AUIPC(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.pc.as_s() + self.inst.imm;
        }
    }

    fn BEQ(&mut self) {
        if self.x[self.inst.rs1 as usize] == self.x[self.inst.rs2 as usize] {
            let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
            if self.is_misaligned(pc) {
                self.eei.trap(Traps::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn BGE(&mut self) {
        if self.x[self.inst.rs1 as usize] >= self.x[self.inst.rs2 as usize] {
            let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
            if self.is_misaligned(pc) {
                self.eei.trap(Traps::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn BGEU(&mut self) {
        if (self.x[self.inst.rs1 as usize].as_u()) >= (self.x[self.inst.rs2 as usize].as_u()) {
            let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
            if self.is_misaligned(pc) {
                self.eei.trap(Traps::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn BLT(&mut self) {
        if self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize] {
            let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
            if self.is_misaligned(pc) {
                self.eei.trap(Traps::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn BLTU(&mut self) {
        if (self.x[self.inst.rs1 as usize].as_u()) < (self.x[self.inst.rs2 as usize].as_u()) {
            let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
            if self.is_misaligned(pc) {
                self.eei.trap(Traps::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn BNE(&mut self) {
        if self.x[self.inst.rs1 as usize] != self.x[self.inst.rs2 as usize] {
            let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
            if self.is_misaligned(pc) {
                self.eei.trap(Traps::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn EBREAK(&mut self) {
        self.eei.trap(Traps::Breakpoint);
    }

    fn ECALL(&mut self) {
        self.eei.trap(Traps::SystemCall);
    }

    fn FENCE(&mut self) {
    }

    fn JAL(&mut self) {
        let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
        if self.is_misaligned(pc) {
            self.eei.trap(Traps::InstructionAddressMisaligned);
        } else {
            self.pc = pc;
            if self.inst.rd != 0 {
                self.x[self.inst.rd as usize] = self.inst.pc.as_s() + 4.into();
            }
        }
    }

    fn JALR(&mut self) {
        let pc = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u() & 0xFFFF_FFFEu32.into();
        if self.is_misaligned(pc) {
            self.eei.trap(Traps::InstructionAddressMisaligned);
        } else {
            self.pc = pc;
            if self.inst.rd != 0 {
                self.x[self.inst.rd as usize] = self.inst.pc.as_s() + 4.into();
            }
        }
    }

    fn LB(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Traps::IllegalInstruction);
        } else {
            let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
            self.x[self.inst.rd as usize] = (self.eei.get_byte(addr) as i8).into();
        }
    }

    fn LBU(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Traps::IllegalInstruction);
        } else {
            let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
            self.x[self.inst.rd as usize] = self.eei.get_byte(addr).into();
        }
    }

    fn LH(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Traps::IllegalInstruction);
        } else {
            let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
            self.x[self.inst.rd as usize] = (self.eei.get_half(addr) as i16).into();
        }
    }

    fn LHU(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Traps::IllegalInstruction);
        } else {
            let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
            self.x[self.inst.rd as usize] = self.eei.get_half(addr).into();
        }
    }

    fn LUI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.imm;
        }
    }

    fn LW(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Traps::IllegalInstruction);
        } else {
            let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
            self.x[self.inst.rd as usize] = (self.eei.get_word(addr) as i32).into();
        }
    }

    fn OR(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] | self.x[self.inst.rs2 as usize];
        }
    }

    fn ORI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] | self.inst.imm;
        }
    }

    fn SB(&mut self) {
        let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
        self.eei.set_byte(addr, self.x[self.inst.rs2 as usize].as_byte());
    }

    fn SH(&mut self) {
        let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
        self.eei.set_half(addr, self.x[self.inst.rs2 as usize].as_half());
    }

    fn SLL(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.x[self.inst.rs2 as usize] & 0x3F.into());
        }
    }

    fn SLLI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.inst.imm & 0x3F.into());
        }
    }

    fn SLT(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize]).into();
        }
    }

    fn SLTI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.inst.imm).into();
        }
    }

    fn SLTIU(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() < self.inst.imm.as_u()).into();
        }
    }

    fn SLTU(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() < self.x[self.inst.rs2 as usize].as_u()).into();
        }
    }

    fn SRA(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.x[self.inst.rs2 as usize] & 0x3F.into());
        }
    }

    fn SRAI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.inst.imm & 0x3Fi32.into());
        }
    }

    fn SRL(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() >> (self.x[self.inst.rs2 as usize].as_u() & 0x3Fu32.into())).as_s();
        }
    }

    fn SRLI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() >> (self.inst.imm.as_u() & 0x3Fu32.into())).as_s();
        }
    }

    fn SUB(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] - self.x[self.inst.rs2 as usize];
        }
    }

    fn SW(&mut self) {
        let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
        self.eei.set_word(addr, self.x[self.inst.rs2 as usize].as_word());
    }

    fn XOR(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] ^ self.x[self.inst.rs2 as usize];
        }
    }

    fn XORI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] ^ self.inst.imm;
        }
    }
}

impl<EEI: ExecutionEnvironmentInterface<u64>> I64 for RV64I<EEI> {
    fn load_execute_i64(&mut self) {
        self.execute[ISA::ADDIW as usize..=ISA::SUBW as usize].copy_from_slice(&Self::EXECUTE_I64);
    }

    fn ADDIW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] + self.inst.imm).as_word() as i32).into();
        }
    }

    fn ADDW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] + self.x[self.inst.rs2 as usize]).as_word() as i32).into();
        }
    }

    fn LD(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Traps::IllegalInstruction);
        } else {
            let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
            self.x[self.inst.rd as usize] = (self.eei.get_double(addr) as i64).into();
        }
    }

    fn LWU(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Traps::IllegalInstruction);
        } else {
            let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
            self.x[self.inst.rd as usize] = self.eei.get_word(addr).into();
        }
    }

    fn SD(&mut self) {
        let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
        self.eei.set_double(addr, self.x[self.inst.rs2 as usize].as_double());
    }

    fn SLLIW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] << (self.inst.imm & 0x1F)).as_word() as i32).into();
        }
    }

    fn SLLW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] << (self.x[self.inst.rs2 as usize] & 0x1F)).as_word() as i32).into();
        }
    }

    fn SRAIW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] >> (self.inst.imm & 0x1F)).as_word() as i32).into();
        }
    }

    fn SRAW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] >> (self.x[self.inst.rs2 as usize] & 0x1F)).as_word() as i32).into();
        }
    }

    fn SRLIW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (((self.x[self.inst.rs1 as usize].as_u() >> (self.inst.imm.as_u() & 0x1F)).as_s()).as_word() as i32).into();
        }
    }

    fn SRLW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (((self.x[self.inst.rs1 as usize].as_u() >> (self.x[self.inst.rs2 as usize].as_u() & 0x1F)).as_s()).as_word() as i32).into();
        }
    }

    fn SUBW(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] - self.x[self.inst.rs2 as usize]).as_word() as i32).into();
        }
    }
}
