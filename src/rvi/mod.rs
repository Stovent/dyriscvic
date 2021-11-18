//! The core module, containing the structs, assembler, disassembler and interpreter.

pub mod assembler;
pub mod disassembler;
mod interpreter;

use crate::common::{*, isa::*};
use crate::public::ExecutionEnvironmentInterface;

/// Configuration of the RISC-V hart.
#[derive(Clone, Debug)]
pub struct RVConfig {
    /// The list of ISA extensions.
    pub ext: String,
    /// Used by the disassembler. See [`get_x_register_name`] and [`get_f_register_name`].
    pub abi_name: bool,
}

/// RISC-V hart.
///
/// `EEI` is the execution environment of the hart, provided by the user.
pub struct RV64I<EEI: ExecutionEnvironmentInterface> {
    /// The integer registers. `x[0]` must always be 0.
    pub x: [i64; 32],
    /// The program counter. It points to the next instruction before executing the current instruction.
    pub pc: u64,

    /// The instruction currently being executed.
    pub inst: Instruction,
    /// Configuration of the context.
    pub config: RVConfig,
    eei: EEI,
    execute: [fn(&mut Self); Isa::_SIZE as usize],
    // disassemble: [fn(Instruction, abi_name: bool) -> String; ISA::_SIZE as usize],
}

impl<EEI: ExecutionEnvironmentInterface> RV64I<EEI> {
    fn is_misaligned(&self, val: u64) -> bool {
        if self.config.ext.contains('C') {
            return !is_even(val);
        } else {
            return val & 0b11 != 0;
        }
    }

    /// Creates a new RV64I hart.
    ///
    /// `x` is the initial state of the registers (`x[0]` is ignored and set to 0).
    /// `pc` is the initial program counter value.
    /// `ext` is a string containing the ISA extensions to be used.
    /// `eei` is the execution environment interface.
    pub fn new(x: [i64; 32], pc: u64, config: RVConfig, eei: EEI) -> Self {
        let mut core = Self {
            x,
            pc,
            inst: Instruction::empty(Isa::UNKNOWN, 0u16.into(), 0),
            config,
            eei,
            execute: [RV64I::UNKNOWN; Isa::_SIZE as usize],
            // disassemble: [RV64I::<EEI>::disassemble_UNKNOWN; ISA::_SIZE as usize],
        };
        core.x[0] = 0;
        // core.load_isa();
        core.execute[Isa::ANDI as usize] = Self::ANDI;
        core.execute[Isa::JAL as usize] = Self::JAL;
        core
    }
}
