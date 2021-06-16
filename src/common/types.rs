use std::ops::*;
use std::fmt::*;

pub trait Int = Copy + Sized + Debug + From<bool> + From<u8> + From<u16> + Ord + UpperHex +
    Add<Output = Self> + AddAssign + Sub<Output = Self> + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Shl<Output = Self> + Shr<Output = Self>;
pub trait Signed<U> = Int + From<i8> + From<i16> + From<i32> + AsUnsigned<Unsigned = U> + As;
pub trait Unsigned<S> = Int + From<u32> + AsSigned<Signed = S> + As;

pub trait AsSigned {
    type Signed;
    fn as_s(&self) -> Self::Signed;
}

impl AsSigned for u32 {
    type Signed = i32;
    fn as_s(&self) -> Self::Signed {
        *self as i32
    }
}

impl AsSigned for u64 {
    type Signed = i64;
    fn as_s(&self) -> Self::Signed {
        *self as i64
    }
}

pub trait AsUnsigned {
    type Unsigned;
    fn as_u(&self) -> Self::Unsigned;
}

impl AsUnsigned for i32 {
    type Unsigned = u32;
    fn as_u(&self) -> Self::Unsigned {
        *self as u32
    }
}

impl AsUnsigned for i64 {
    type Unsigned = u64;
    fn as_u(&self) -> Self::Unsigned {
        *self as u64
    }
}

pub trait As {
    fn as_byte(&self) -> u8;
    fn as_half(&self) -> u16;
    fn as_word(&self) -> u32;
}

impl As for u32 {
    fn as_byte(&self) -> u8 { *self as u8 }
    fn as_half(&self) -> u16 { *self as u16 }
    fn as_word(&self) -> u32 { *self }
}

impl As for i32 {
    fn as_byte(&self) -> u8 { *self as u8 }
    fn as_half(&self) -> u16 { *self as u16 }
    fn as_word(&self) -> u32 { *self as u32 }
}

impl As for u64 {
    fn as_byte(&self) -> u8 { *self as u8 }
    fn as_half(&self) -> u16 { *self as u16 }
    fn as_word(&self) -> u32 { *self as u32 }
}

impl As for i64 {
    fn as_byte(&self) -> u8 { *self as u8 }
    fn as_half(&self) -> u16 { *self as u16 }
    fn as_word(&self) -> u32 { *self as u32 }
}
