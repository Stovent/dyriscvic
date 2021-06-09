pub mod disassemble;
pub mod execute;

use crate::common::{*, instruction::*, isa::*, types::*};
use crate::public::ExecutionEnvironmentInterface;

pub struct RVI<U: Unsigned<S>, S: Signed<U>, const N: usize> {
    x: [S; N],
    pc: U,

    inst: Instruction<U, S>,
    pub ext: String,
    eei: Box<dyn ExecutionEnvironmentInterface<U>>,
    execute: [fn(&mut Self); ISA::_SIZE as usize],
    disassemble: [fn(Instruction<U, S>) -> String; ISA::_SIZE as usize],
}

pub type RV32<const N: usize> = RVI<u32, i32, N>;
pub type RV32E = RV32<16>;
pub type RV32I = RV32<32>;
pub type RV64I = RVI<u64, i64, 32>;

impl<U: Unsigned<S>, S: Signed<U>, const N: usize> RVI<U, S, N> {
    pub fn new(x: [S; N], pc: U, ext: &str, eei: impl ExecutionEnvironmentInterface<U> + 'static) -> Self {
        let mut core = Self {
            x,
            pc,
            inst: Instruction::<U, S>::new_empty(ISA::UNKNOWN, 0u16.into()),
            ext: String::from(ext).to_ascii_uppercase(),
            eei: Box::new(eei),
            execute: [RVI::UNKNOWN; ISA::_SIZE as usize],
            disassemble: [RVI::<U, S, N>::disassemble_UNKNOWN; ISA::_SIZE as usize],
        };
        core.x[0] = 0.into();
        core.load_execute_i32();
        core.load_disassemble_i32();
        core
    }

    fn is_misaligned(&self, val: U) -> bool {
        if self.ext.contains('C') {
            return !is_even(val);
        } else {
            return val & 0b11u32.into() != 0u32.into();
        }
    }

    pub fn single_step(&mut self) {
        let pc = self.pc;
        self.pc += 4u32.into();
        let opcode = self.eei.get32le(pc); // TODO: instruction-address-misaligned
        let inst_size = get_instruction_length(opcode as u16);
        match inst_size {
//            2 => if self.ext.contains('C'),
            4 => {
                self.inst = Instruction::<U, S>::from_opcode_32(pc, opcode);

                #[cfg(debug_assertions)]
                println!("Instruction: {}", self.disassemble[self.inst.inst as usize](self.inst));

                self.execute[self.inst.inst as usize](self);
            },
            _ => println!("Unknown opcode {:#X} at {:#X}", opcode, pc),
        };
    }
}
