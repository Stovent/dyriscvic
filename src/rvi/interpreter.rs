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
//            2 => if self.ext.contains('C'),
            4 => {
                self.inst = Instruction::from_opcode_32(pc, opcode);

                // #[cfg(debug_assertions)]
                // println!("Instruction: {}", self.disassemble[self.inst.inst as usize](self.inst, self.config.abi_name));

                self.execute[self.inst.inst as usize](self);
            },
            _ => println!("Unknown opcode {:#X} at {:#X}", opcode, pc),
        };
    }
}

impl<EEI: ExecutionEnvironmentInterface> I32<EEI> for RV64I<EEI> {
    fn load_execute_i32(&mut self) {
        self.execute[Isa::ADD as usize..=Isa::XORI as usize].copy_from_slice(&RV64I::<EEI>::EXECUTE_I32);
    }

    fn UNKNOWN(&mut self) {
        self.eei.trap(Trap::IllegalInstruction);
    }

    fn ADD(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] + self.x[self.inst.rs2 as usize];
        }
    }

    fn ADDI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] + self.inst.imm as i64;
        }
    }

    fn AND(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] & self.x[self.inst.rs2 as usize];
        }
    }

    fn ANDI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] & self.inst.imm as i64;
        }
    }

    fn AUIPC(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = self.inst.pc as i64 + self.inst.imm as i64;
        }
    }

    fn BEQ(&mut self) {
        if self.x[self.inst.rs1 as usize] == self.x[self.inst.rs2 as usize] {
            // let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
            // if self.is_misaligned(pc) {
            //     self.eei.trap(Traps::InstructionAddressMisaligned);
            // } else {
            //     self.pc = pc;
            // }
        }
    }

    fn BGE(&mut self) {
        if self.x[self.inst.rs1 as usize] >= self.x[self.inst.rs2 as usize] {
            // let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
            // if self.is_misaligned(pc) {
            //     self.eei.trap(Traps::InstructionAddressMisaligned);
            // } else {
            //     self.pc = pc;
            // }
        }
    }

    fn BGEU(&mut self) {
        // if (self.x[self.inst.rs1 as usize].as_u()) >= (self.x[self.inst.rs2 as usize].as_u()) {
        //     let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
        //     if self.is_misaligned(pc) {
        //         self.eei.trap(Traps::InstructionAddressMisaligned);
        //     } else {
        //         self.pc = pc;
        //     }
        // }
    }

    fn BLT(&mut self) {
        if self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize] {
            // let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
            // if self.is_misaligned(pc) {
            //     self.eei.trap(Traps::InstructionAddressMisaligned);
            // } else {
            //     self.pc = pc;
            // }
        }
    }

    fn BLTU(&mut self) {
        // if (self.x[self.inst.rs1 as usize].as_u()) < (self.x[self.inst.rs2 as usize].as_u()) {
        //     let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
        //     if self.is_misaligned(pc) {
        //         self.eei.trap(Traps::InstructionAddressMisaligned);
        //     } else {
        //         self.pc = pc;
        //     }
        // }
    }

    fn BNE(&mut self) {
        if self.x[self.inst.rs1 as usize] != self.x[self.inst.rs2 as usize] {
            // let pc = (self.inst.pc.as_s() + self.inst.imm as i64).as_u();
            // if self.is_misaligned(pc) {
            //     self.eei.trap(Traps::InstructionAddressMisaligned);
            // } else {
            //     self.pc = pc;
            // }
        }
    }

    fn EBREAK(&mut self) {
        self.eei.trap(Trap::Breakpoint);
    }

    fn ECALL(&mut self) {
        self.eei.trap(Trap::SystemCall);
    }

    fn FENCE(&mut self) {
    }

    fn JAL(&mut self) {
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

    fn JALR(&mut self) {
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

    fn LB(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Trap::IllegalInstruction);
        } else {
            // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
            // self.x[self.inst.rd as usize] = (self.eei.get_8(addr) as i8).into();
        }
    }

    fn LBU(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Trap::IllegalInstruction);
        } else {
            // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
            // self.x[self.inst.rd as usize] = self.eei.get_8(addr).into();
        }
    }

    fn LH(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Trap::IllegalInstruction);
        } else {
            // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
            // self.x[self.inst.rd as usize] = (self.eei.get_16(addr) as i16).into();
        }
    }

    fn LHU(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Trap::IllegalInstruction);
        } else {
            // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
            // self.x[self.inst.rd as usize] = self.eei.get_16(addr).into();
        }
    }

    fn LUI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.imm as i64;
        }
    }

    fn LW(&mut self) {
        if self.inst.rd == 0 {
            self.eei.trap(Trap::IllegalInstruction);
        } else {
            // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
            // self.x[self.inst.rd as usize] = (self.eei.get_32(addr) as i32).into();
        }
    }

    fn OR(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] | self.x[self.inst.rs2 as usize];
        }
    }

    fn ORI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] | self.inst.imm as i64;
        }
    }

    fn SB(&mut self) {
        // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
        // self.eei.set_8(addr, self.x[self.inst.rs2 as usize].as_u8());
    }

    fn SH(&mut self) {
        // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
        // self.eei.set_16(addr, self.x[self.inst.rs2 as usize].as_u16());
    }

    fn SLL(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.x[self.inst.rs2 as usize] & 0x3F.into());
        }
    }

    fn SLLI(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.inst.imm & 0x3F.into());
        }
    }

    fn SLT(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize]).into();
        }
    }

    fn SLTI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.inst.imm as i64).into();
        }
    }

    fn SLTIU(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() < (self.inst.imm as i64).as_u()).into();
        }
    }

    fn SLTU(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() < self.x[self.inst.rs2 as usize].as_u()).into();
        }
    }

    fn SRA(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.x[self.inst.rs2 as usize] & 0x3F.into());
        }
    }

    fn SRAI(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.inst.imm & 0x3Fi32.into());
        }
    }

    fn SRL(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() >> (self.x[self.inst.rs2 as usize].as_u() & 0x3Fu32.into())).as_s();
        }
    }

    fn SRLI(&mut self) {
        if self.inst.rd != 0 {
            // self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize].as_u() >> (self.inst.imm.as_u() & 0x3Fu32.into())).as_s();
        }
    }

    fn SUB(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] - self.x[self.inst.rs2 as usize];
        }
    }

    fn SW(&mut self) {
        // let addr = (self.x[self.inst.rs1 as usize] + self.inst.imm as i64).as_u();
        // self.eei.set_32(addr, self.x[self.inst.rs2 as usize].as_u32());
    }

    fn XOR(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] ^ self.x[self.inst.rs2 as usize];
        }
    }

    fn XORI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] ^ self.inst.imm as i64;
        }
    }
}
