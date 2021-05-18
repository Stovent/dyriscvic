use crate::common::extensions::*;
use crate::rvi::*;

impl<'a, const N: usize> I32 for RV32<'a, N> {
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
            self.x[self.inst.rd as usize] = self.inst.pc as i32 + self.inst.imm;
        }
    }

    fn BEQ(&mut self) {
        if self.x[self.inst.rs1 as usize] == self.x[self.inst.rs2 as usize] {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    fn BGE(&mut self) {
        if self.x[self.inst.rs1 as usize] >= self.x[self.inst.rs2 as usize] {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    fn BGEU(&mut self) {
        if (self.x[self.inst.rs1 as usize] as u32) >= (self.x[self.inst.rs2 as usize] as u32) {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    fn BLT(&mut self) {
        if self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize] {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    fn BLTU(&mut self) {
        if (self.x[self.inst.rs1 as usize] as u32) < (self.x[self.inst.rs2 as usize] as u32) {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    fn BNE(&mut self) {
        if self.x[self.inst.rs1 as usize] != self.x[self.inst.rs2 as usize] {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    fn EBREAK(&mut self) {
    }

    fn ECALL(&mut self) {
    }

    fn FENCE(&mut self) {
    }

    fn JAL(&mut self) {
        self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.pc as i32 + 4;
        }
    }

    fn JALR(&mut self) {
        self.pc = (self.x[self.inst.rs1 as usize] + self.inst.imm) as u32 & 0xFFFF_FFFE;
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.pc as i32 + 4;
        }
    }

    fn LB(&mut self) {
        // if rd == 0, throw exception
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = self.eei.get8((self.x[self.inst.rs1 as usize] + self.inst.imm) as u64) as i8 as i32;
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
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.x[self.inst.rs2 as usize] & 0x1F);
        }
    }

    fn SLLI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.inst.imm & 0x1F);
        }
    }

    fn SLT(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize]) as i32;
        }
    }

    fn SLTI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.inst.imm) as i32;
        }
    }

    fn SLTIU(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as u32) < (self.inst.imm as u32)) as i32;
        }
    }

    fn SLTU(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as u32) < self.x[self.inst.rs2 as usize] as u32) as i32;
        }
    }

    fn SRA(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.x[self.inst.rs2 as usize] & 0x1F);
        }
    }

    fn SRAI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.inst.imm & 0x1F);
        }
    }

    fn SRL(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as u32) >> (self.x[self.inst.rs2 as usize] as u32 & 0x1F)) as i32;
        }
    }

    fn SRLI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] as u32 >> (self.inst.imm as u32 & 0x1F)) as i32;
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
