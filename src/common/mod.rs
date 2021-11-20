//! Traits, enums and structs shared betwenn the library's components and that can be useful to the user.

pub mod decoder;
pub mod instruction;
pub mod isa;

pub use instruction::Instruction;

/// Returns true if the given number is even, false if it odd.
pub const fn is_even(num: u64) -> bool {
    return num & 1 == 0;
}

/// Converts u16 and u32 to byte slices, in little-endian.
///
/// `slice[0]` will have the LSB and `slice[N - 1]` the MSB.
pub trait AsSlice<const N: usize> {
    fn as_slice_le(self) -> [u8; N];
}

impl AsSlice<2> for u16 {
    fn as_slice_le(self) -> [u8; 2] {
        [self as u8, (self >> 8) as u8]
    }
}

impl AsSlice<4> for u32 {
    fn as_slice_le(self) -> [u8; 4] {
        [self as u8, (self >> 8) as u8, (self >> 16) as u8, (self >> 24) as u8]
    }
}

/// Get the integer register name associated with its number.
///
/// If `abi_name` is true, returns the ABI name as in table 25.1. Otherwise returns `x0`, `x1`, etc.
pub fn get_x_register_name(reg: u8, abi_name: bool) -> String {
    if abi_name {
        String::from(match reg {
            0 => "zero",
            1 => "ra",
            2 => "sp",
            3 => "gp",
            4 => "tp",
            5 => "t0",
            6 => "t1",
            7 => "t2",
            8 => "s0",
            9 => "s1",
            10 => "a0",
            11 => "a1",
            12 => "a2",
            13 => "a3",
            14 => "a4",
            15 => "a5",
            16 => "a6",
            17 => "a7",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "s8",
            25 => "s9",
            26 => "s10",
            27 => "s11",
            28 => "t3",
            29 => "t4",
            30 => "t5",
            31 => "t6",
            _ => "Unknown register"
        })
    } else {
        format!("x{}", reg)
    }
}

/// Get the floating point register name associated with its number.
///
/// If `abi_name` is true, returns the ABI name as in table 25.1. Otherwise returns `f0`, `f1`, etc.
pub fn get_f_register_name(reg: u8, abi_name: bool) -> String {
    if abi_name {
        String::from(match reg {
            0 => "ft0",
            1 => "ft1",
            2 => "ft2",
            3 => "ft3",
            4 => "ft4",
            5 => "ft5",
            6 => "ft6",
            7 => "ft7",
            8 => "fs0",
            9 => "fs1",
            10 => "fa0",
            11 => "fa1",
            12 => "fa2",
            13 => "fa3",
            14 => "fa4",
            15 => "fa5",
            16 => "fa6",
            17 => "fa7",
            18 => "fs2",
            19 => "fs3",
            20 => "fs4",
            21 => "fs5",
            22 => "fs6",
            23 => "fs7",
            24 => "fs8",
            25 => "fs9",
            26 => "fs10",
            27 => "fs11",
            28 => "ft8",
            29 => "ft9",
            30 => "ft10",
            31 => "ft11",
            _ => "Unknown register"
        })
    } else {
        format!("f{}", reg)
    }
}
