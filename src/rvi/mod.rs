//! The core module, containing the structs, assembler, disassembler and interpreter.

pub mod assembler;
pub mod disassembler;
mod interpreter;

use crate::common::{*, instruction::*, isa::*};
use crate::public::ExecutionEnvironmentInterface;

/// Configuration of the RISC-V hart.
#[derive(Clone, Debug)]
pub struct RVConfig {
    /// The list of ISA extensions.
    pub ext: String,
    /// Used by the disassembler. See [`get_x_register_name`] and [`get_f_register_name`].
    pub abi_name: bool,
}

macro_rules! declare_rvi {
    ($name:ident, $regtype:ty, $pctype:ty, $entry:ident, $inst:ty) => {
        /// RISC-V hart.
        ///
        /// `EEI` is the execution environment of the hart, provided by the user.
        pub struct $name<EEI: ExecutionEnvironmentInterface> {
            /// The integer registers. `x[0]` must always be 0.
            pub x: [$regtype; 32],
            /// The program counter. It points to the next instruction before executing the current instruction.
            pub pc: $pctype,

            /// The instruction currently being executed.
            pub inst: $inst,
            /// Configuration of the context.
            pub config: RVConfig,
            eei: EEI,
            isa: [$entry<EEI>; Isa::_SIZE as usize],
        }
    };
}

declare_rvi!(RV32I, i32, u32, IsaEntry32, Instruction32);
declare_rvi!(RV64I, i64, u64, IsaEntry64, Instruction64);

macro_rules! impl_rvi {
    ($name:ident, $regtype:ty, $pctype:ty, $entry:ident, $inst:ty) => {
        impl<EEI: ExecutionEnvironmentInterface> $name<EEI> {
            fn is_misaligned(&self, val: $pctype) -> bool {
                if self.config.ext.contains('C') {
                    return !is_even(val as u64);
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
            pub fn new(x: [$regtype; 32], pc: $pctype, config: RVConfig, eei: EEI) -> Self {
                let mut core = Self {
                    x,
                    pc,
                    inst: <$inst>::empty(Isa::UNKNOWN, 0u32.into(), 0),
                    config,
                    eei,
                    isa: [Self::ISA_UNKNOWN; Isa::_SIZE as usize],
                };
                core.load_isa();
                core.x[0] = 0;
                core
            }
        }
    };
}

impl_rvi!(RV32I, i32, u32, IsaEntry32, Instruction32);
impl_rvi!(RV64I, i64, u64, IsaEntry64, Instruction64);

impl<EEI: ExecutionEnvironmentInterface> RV32I<EEI> {
    fn load_isa(&mut self) {
        self.isa[Isa::ADD as usize..Isa::XORI as usize + 1].copy_from_slice(&Self::ISA_LUT_I32);
    }
}

impl<EEI: ExecutionEnvironmentInterface> RV64I<EEI> {
    fn load_isa(&mut self) {
        self.isa[Isa::ADD as usize..Isa::XORI as usize + 1].copy_from_slice(&Self::ISA_LUT_I32);
        self.isa[Isa::ADDIW as usize..Isa::SUBW as usize + 1].copy_from_slice(&Self::ISA_LUT_I64);
    }
}
