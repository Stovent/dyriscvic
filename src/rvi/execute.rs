use crate::common::isa::*;
use crate::public::*;
use crate::rvi::*;

impl<U: Unsigned<S>, S: Signed<U>, EEI: ExecutionEnvironmentInterface<U>, const N: usize> I32<U, S, EEI, N> for RVI<U, S, EEI, N> {
    fn load_execute_i32(&mut self) {
        self.execute[ISA::ADD as usize..=ISA::XORI as usize].copy_from_slice(&RVI::<U, S, EEI, N>::EXECUTE_I32);
    }

    fn UNKNOWN(&mut self) {
        self.eei.exception(Exceptions::IllegalInstruction);
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
                self.eei.exception(Exceptions::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn BGE(&mut self) {
        if self.x[self.inst.rs1 as usize] >= self.x[self.inst.rs2 as usize] {
            let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
            if self.is_misaligned(pc) {
                self.eei.exception(Exceptions::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn BGEU(&mut self) {
        if (self.x[self.inst.rs1 as usize].as_u()) >= (self.x[self.inst.rs2 as usize].as_u()) {
            let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
            if self.is_misaligned(pc) {
                self.eei.exception(Exceptions::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn BLT(&mut self) {
        if self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize] {
            let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
            if self.is_misaligned(pc) {
                self.eei.exception(Exceptions::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn BLTU(&mut self) {
        if (self.x[self.inst.rs1 as usize].as_u()) < (self.x[self.inst.rs2 as usize].as_u()) {
            let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
            if self.is_misaligned(pc) {
                self.eei.exception(Exceptions::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn BNE(&mut self) {
        if self.x[self.inst.rs1 as usize] != self.x[self.inst.rs2 as usize] {
            let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
            if self.is_misaligned(pc) {
                self.eei.exception(Exceptions::InstructionAddressMisaligned);
            } else {
                self.pc = pc;
            }
        }
    }

    fn EBREAK(&mut self) {
    }

    fn ECALL(&mut self) {
    }

    fn FENCE(&mut self) {
    }

    fn JAL(&mut self) {
        let pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
        if self.is_misaligned(pc) {
            self.eei.exception(Exceptions::InstructionAddressMisaligned);
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
            self.eei.exception(Exceptions::InstructionAddressMisaligned);
        } else {
            self.pc = pc;
            if self.inst.rd != 0 {
                self.x[self.inst.rd as usize] = self.inst.pc.as_s() + 4.into();
            }
        }
    }

    fn LB(&mut self) {
        let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
        if self.is_misaligned(addr) {
            self.eei.exception(Exceptions::InstructionAddressMisaligned);
        } else if self.inst.rd == 0 {
            self.eei.exception(Exceptions::IllegalInstruction);
        } else {
            self.x[self.inst.rd as usize] = (self.eei.get8(addr) as i8).into();
        }
    }

    fn LBU(&mut self) {
        let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
        if self.is_misaligned(addr) {
            self.eei.exception(Exceptions::InstructionAddressMisaligned);
        } else if self.inst.rd == 0 {
            self.eei.exception(Exceptions::IllegalInstruction);
        } else {
            self.x[self.inst.rd as usize] = self.eei.get8(addr).into();
        }
    }

    fn LH(&mut self) {
        let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
        if self.is_misaligned(addr) {
            self.eei.exception(Exceptions::InstructionAddressMisaligned);
        } else if self.inst.rd == 0 {
            self.eei.exception(Exceptions::IllegalInstruction);
        } else {
            self.x[self.inst.rd as usize] = (self.eei.get16(addr) as i16).into();
        }
    }

    fn LHU(&mut self) {
        let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
        if self.is_misaligned(addr) {
            self.eei.exception(Exceptions::InstructionAddressMisaligned);
        } else if self.inst.rd == 0 {
            self.eei.exception(Exceptions::IllegalInstruction);
        } else {
            self.x[self.inst.rd as usize] = self.eei.get16(addr).into();
        }
    }

    fn LUI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.imm;
        }
    }

    fn LW(&mut self) {
        let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
        if self.is_misaligned(addr) {
            self.eei.exception(Exceptions::InstructionAddressMisaligned);
        } else if self.inst.rd == 0 {
            self.eei.exception(Exceptions::IllegalInstruction);
        } else {
            self.x[self.inst.rd as usize] = (self.eei.get32(addr) as i32).into();
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
        if self.is_misaligned(addr) {
            self.eei.exception(Exceptions::InstructionAddressMisaligned);
        } else {
            self.eei.set8(addr, self.x[self.inst.rs2 as usize].as_byte());
        }
    }

    fn SH(&mut self) {
        let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
        if self.is_misaligned(addr) {
            self.eei.exception(Exceptions::InstructionAddressMisaligned);
        } else {
            self.eei.set16(addr, self.x[self.inst.rs2 as usize].as_half());
        }
    }

    fn SLL(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.x[self.inst.rs2 as usize] & 0x1F.into());
        }
    }

    fn SLLI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.inst.imm & 0x1F.into());
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
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.x[self.inst.rs2 as usize] & 0x1F.into());
        }
    }

    fn SRAI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.inst.imm & 0x1Fi32.into());
        }
    }

    fn SRL(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() >> (self.x[self.inst.rs2 as usize].as_u() & 0x1Fu32.into())).as_s();
        }
    }

    fn SRLI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() >> (self.inst.imm.as_u() & 0x1Fu32.into())).as_s();
        }
    }

    fn SUB(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] - self.x[self.inst.rs2 as usize];
        }
    }

    fn SW(&mut self) {
        let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u();
        if self.is_misaligned(addr) {
            self.eei.exception(Exceptions::InstructionAddressMisaligned);
        } else {
            self.eei.set32(addr, self.x[self.inst.rs2 as usize].as_word());
        }
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
    }

    fn ADDW(&mut self) {
    }

    fn LD(&mut self) {
    }

    fn LWU(&mut self) {
    }

    fn SD(&mut self) {
    }

    fn SLLIW(&mut self) {
    }

    fn SLLW(&mut self) {
    }

    fn SRAIW(&mut self) {
    }

    fn SRAW(&mut self) {
    }

    fn SRLIW(&mut self) {
    }

    fn SRLW(&mut self) {
    }

    fn SUBW(&mut self) {
    }

}
