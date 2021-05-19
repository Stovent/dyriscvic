pub mod disassemble;
pub mod execute;

use crate::common::{*, extensions::*, isa::*, types::*};
use crate::public::ExecutionEnvironmentInterface;

pub struct RVI<U: Unsigned<S>, S: Signed<U>, const N: usize> {
    x: [S; N],
    pc: U,

    inst: Instruction<U, S>,
    pub ext: String,
    eei: Box<dyn ExecutionEnvironmentInterface<U>>,
    execute: [fn(&mut Self); 41],
    disassemble: [fn(Instruction<U, S>); 41],
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
            inst: Instruction::<U, S>::new_empty(ISA::UNKNOWN, 0.into()),
            ext: String::from(ext),
            eei: Box::new(eei),
            execute: [RVI::UNKNOWN; 41],
            disassemble: [RVI::<U, S, N>::disassemble_UNKNOWN; 41],
        };
        core.load_execute_i();
        core.load_disassemble_i();
        core
    }
}

impl RV32I {
    pub fn single_step(&mut self) {
        let pc = self.pc;
        self.pc += 4;
        let opcode = self.eei.get32le(pc);
        let inst_size = get_instruction_length(opcode as u16);
        match inst_size {
//            2 => if self.ext.contains('C'),
            4 => {
                self.inst = Instruction32::get_instruction_from_opcode(pc, opcode);

                #[cfg(debug_assertions)]
                self.disassemble[self.inst.inst as usize](self.inst);

                self.execute[self.inst.inst as usize](self);
            },
            _ => println!("Unknown opcode {:#X} at {:#X}", opcode, pc),
        };
    }
}

impl<U: Unsigned<S>, S: Signed<U>, const N: usize> Execute<U, S, N> for RVI<U, S, N> {}
impl<U: Unsigned<S>, S: Signed<U>, const N: usize> Disassemble<U, S, N> for RVI<U, S, N> {}
