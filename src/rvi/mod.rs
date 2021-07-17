//! The core module, containing the structs, assembler, disassembler and interpreter.

pub mod assembler;
pub mod disassembler;
mod interpreter;

use crate::common::{*, instruction::*, isa::*, types::*};
use crate::public::ExecutionEnvironmentInterface;

/// Configuration of the RISC-V hart.
#[derive(Clone, Debug)]
pub struct RVConfig {
    /// The list of ISA extensions.
    pub ext: String,
    /// Used by the disassembler. See [`get_x_register_name`] and [`get_f_register_name`].
    pub abi_name: bool,
}

/// Struct representing a RISC-V hart.
///
/// `U` is the program counter type (u32 or u64).
/// `S` is the register type (i32 or i64).
/// `EEI` is the execution environment of the hart, provided by the user.
/// `N` is the number of integer registers (16 or 32).
pub struct RVI<U: Unsigned<S>, S: Signed<U>, EEI: ExecutionEnvironmentInterface<U>, const N: usize> {
    /// The integer registers. `x[0]` must always be 0.
    pub x: [S; N],
    /// The program counter. It points to the next instruction before executing the current instruction.
    pub pc: U,

    /// The instruction currently being executed.
    pub inst: Instruction<U, S>,
    /// Configuration of the context.
    pub config: RVConfig,
    eei: EEI,
    execute: [fn(&mut Self); ISA::_SIZE as usize],
    disassemble: [fn(Instruction<U, S>, abi_name: bool) -> String; ISA::_SIZE as usize],
}

/// Convenient alias defining a RV32 hart.
pub type RV32<EEI: ExecutionEnvironmentInterface<u32>, const N: usize> = RVI<u32, i32, EEI, N>;
/// Convenient alias defining a RV32E hart.
pub type RV32E<EEI: ExecutionEnvironmentInterface<u32>> = RV32<EEI, 16>;
/// Convenient alias defining a RV32I hart.
pub type RV32I<EEI: ExecutionEnvironmentInterface<u32>> = RV32<EEI, 32>;
/// Convenient alias defining a RV64I hart.
pub type RV64I<EEI: ExecutionEnvironmentInterface<u64>> = RVI<u64, i64, EEI, 32>;

impl<U: Unsigned<S>, S: Signed<U>, EEI: ExecutionEnvironmentInterface<U>, const N: usize> RVI<U, S, EEI, N> {
    fn is_misaligned(&self, val: U) -> bool {
        if self.config.ext.contains('C') {
            return !is_even(val);
        } else {
            return val & 0b11u32.into() != 0u32.into();
        }
    }

    /// Executes a single intruction on the hart.
    pub fn single_step(&mut self) {
        let pc = self.pc;
        self.pc += 4u32.into();
        let opcode = self.eei.get_opcode_32(pc); // TODO: instruction-address-misaligned
        let inst_size = get_instruction_length(opcode as u16);
        match inst_size {
//            2 => if self.ext.contains('C'),
            4 => {
                self.inst = Instruction::<U, S>::from_opcode_32(pc, opcode);

                #[cfg(debug_assertions)]
                println!("Instruction: {}", self.disassemble[self.inst.inst as usize](self.inst, self.config.abi_name));

                self.execute[self.inst.inst as usize](self);
            },
            _ => println!("Unknown opcode {:#X} at {:#X}", opcode, pc),
        };
    }
}

impl<EEI: ExecutionEnvironmentInterface<u32>, const N: usize> RV32<EEI, N> {
    /// Creates a new RV32I/E hart.
    ///
    /// `x` is the initial state of the registers. `x[0]` will be set to 0 anyway. and must stay at 0.
    /// `pc` is the initial program counter value.
    /// `ext` is a string containing the ISA extensions to be used.
    /// `eei` is the execution environment interface.
    pub fn new(x: [i32; N], pc: u32, config: RVConfig, eei: EEI) -> Self {
        let mut core = Self {
            x,
            pc,
            inst: Instruction32::empty(ISA::UNKNOWN, 0u16.into(), 0),
            config,
            eei,
            execute: [RVI::UNKNOWN; ISA::_SIZE as usize],
            disassemble: [RV32::<EEI, N>::disassemble_UNKNOWN; ISA::_SIZE as usize],
        };
        core.x[0] = 0.into();
        core.load_isa();
        core
    }
}

impl<EEI: ExecutionEnvironmentInterface<u64>> RV64I<EEI> {
    /// Creates a new RV64I hart.
    ///
    /// `x` is the initial state of the registers. `x[0]` will be set to 0 anyway. and must stay at 0.
    /// `pc` is the initial program counter value.
    /// `ext` is a string containing the ISA extensions to be used.
    /// `eei` is the execution environment interface.
    pub fn new(x: [i64; 32], pc: u64, config: RVConfig, eei: EEI) -> Self {
        let mut core = Self {
            x,
            pc,
            inst: Instruction64::empty(ISA::UNKNOWN, 0u16.into(), 0),
            config,
            eei,
            execute: [RVI::UNKNOWN; ISA::_SIZE as usize],
            disassemble: [RV64I::<EEI>::disassemble_UNKNOWN; ISA::_SIZE as usize],
        };
        core.x[0] = 0.into();
        core.load_isa();
        core
    }
}

trait LoadISA {
    fn load_isa(&mut self);
}

impl<EEI: ExecutionEnvironmentInterface<u32>, const N: usize> LoadISA for RV32<EEI, N> {
    fn load_isa(&mut self) {
        self.load_execute_i32();
        self.load_disassemble_i32();
    }
}

impl<EEI: ExecutionEnvironmentInterface<u64>> LoadISA for RV64I<EEI> {
    fn load_isa(&mut self) {
        self.load_execute_i32();
        self.load_disassemble_i32();
        self.load_execute_i64();
        self.load_disassemble_i64();
    }
}
