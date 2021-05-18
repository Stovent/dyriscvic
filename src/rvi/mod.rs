pub mod disassemble;
pub mod execute;

use crate::common::{*, isa::*, types::*};
use crate::public::ExecutionEnvironmentInterface;

pub struct RVI<'a, PC, X, const N: usize> {
    pub x: [X; N],
    pub pc: PC,

    pub inst: Instruction<PC, X>,
    pub ext: String,
    pub eei: &'a mut dyn ExecutionEnvironmentInterface,
}

pub type RV32<'a, const N: usize> = RVI<'a, u32, i32, N>;
pub type RV32E<'a> = RV32<'a, 16>;
pub type RV32I<'a> = RV32<'a, 32>;
pub type RV64I<'a> = RVI<'a, u64, i64, 32>;

impl<'a> RV32I<'a> {
    pub fn single_step(&mut self) {
        let pc = self.pc;
        self.pc += 4;
        let opcode = self.eei.get32le(pc as u64);
        let inst_size = get_instruction_length(opcode as u16);
        match inst_size {
//            2 => if self.ext.contains('C'),
            4 => {
                self.inst = Instruction32::get_instruction_from_opcode(pc, opcode);

                #[cfg(debug_assertions)]
                RV32I::DISASSEMBLE[self.inst.inst as usize](self.inst);

                RV32I::EXECUTE[self.inst.inst as usize](self);
            },
            _ => println!("Unknown opcode {:#X} at {:#x}", opcode, pc),
        };
    }
}

impl<'a, const N: usize> Execute<'a, N> for RV32<'a, N> {}
impl<'a, PC: Unsigned, IMM: Signed, const N: usize> Disassemble<'a, PC, IMM, N> for RV32<'a, N> {}
