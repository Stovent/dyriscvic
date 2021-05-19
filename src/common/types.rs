use std::ops::*;

pub trait Int = Copy + Sized + std::fmt::Debug + 'static + Add<Output = Self>;
pub trait Signed<U> = Int + From<i32> + AsUnsigned<Unsigned = U>;
pub trait Unsigned<S> = Int + From<u32> + AsSigned<Signed = S>;

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
