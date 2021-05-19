use crate::common::{extensions::*, isa::*};
use crate::rvi::*;

impl<U: Unsigned<S>, S: Signed<U>, const N: usize> I32 for RVI<U, S, N> {
    fn load_execute_i(&mut self) {
        self.execute[ISA::ADD as usize..=ISA::XORI as usize].copy_from_slice(&RVI::<U, S, N>::EXECUTE);
    }

    fn UNKNOWN(&mut self) {}

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
            self.pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
        }
    }

    fn BGE(&mut self) {
        if self.x[self.inst.rs1 as usize] >= self.x[self.inst.rs2 as usize] {
            self.pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
        }
    }

    fn BGEU(&mut self) {
        if (self.x[self.inst.rs1 as usize].as_u()) >= (self.x[self.inst.rs2 as usize].as_u()) {
            self.pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
        }
    }

    fn BLT(&mut self) {
        if self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize] {
            self.pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
        }
    }

    fn BLTU(&mut self) {
        if (self.x[self.inst.rs1 as usize].as_u()) < (self.x[self.inst.rs2 as usize].as_u()) {
            self.pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
        }
    }

    fn BNE(&mut self) {
        if self.x[self.inst.rs1 as usize] != self.x[self.inst.rs2 as usize] {
            self.pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
        }
    }

    fn EBREAK(&mut self) {
    }

    fn ECALL(&mut self) {
    }

    fn FENCE(&mut self) {
    }

    fn JAL(&mut self) {
        self.pc = (self.inst.pc.as_s() + self.inst.imm).as_u();
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.pc.as_s() + 4.into();
        }
    }

    fn JALR(&mut self) {
        self.pc = (self.x[self.inst.rs1 as usize] + self.inst.imm).as_u() & 0xFFFF_FFFE.into();
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.pc.as_s() + 4.into();
        }
    }

    fn LB(&mut self) {
        // if rd == 0, throw exception
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.eei.get8((self.x[self.inst.rs1 as usize] + self.inst.imm).as_u()) as i8).into();
        }
    }

    fn LBU(&mut self) {
    }

    fn LH(&mut self) {
    }

    fn LHU(&mut self) {
    }

    fn LUI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.imm;
        }
    }

    fn LW(&mut self) {
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
    }

    fn SH(&mut self) {
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
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.inst.imm & 0x1F.into());
        }
    }

    fn SRL(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() >> self.x[self.inst.rs2 as usize].as_u() & 0x1F.into()).as_s();
        }
    }

    fn SRLI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() >> (self.inst.imm.as_u() & 0x1F.into())).as_s();
        }
    }

    fn SUB(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] - self.x[self.inst.rs2 as usize];
        }
    }

    fn SW(&mut self) {
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
