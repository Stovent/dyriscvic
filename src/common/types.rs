
pub trait Signed: Copy + From<i32> + Sized + IntoUnsigned {}
pub trait Unsigned: Copy + From<u32> + Sized + IntoSigned {}

impl Signed for i32 {}
impl Signed for i64 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}

pub trait IntoSigned {
    type Signed;
    fn into_s(&self) -> Self::Signed;
}

impl IntoSigned for u32 {
    type Signed = i32;
    fn into_s(&self) -> Self::Signed {
        *self as i32
    }
}

impl IntoSigned for u64 {
    type Signed = i64;
    fn into_s(&self) -> Self::Signed {
        *self as i64
    }
}

pub trait IntoUnsigned {
    type Unsigned;
    fn into_u(&self) -> Self::Unsigned;
}

impl IntoUnsigned for i32 {
    type Unsigned = u32;
    fn into_u(&self) -> Self::Unsigned {
        *self as u32
    }
}

impl IntoUnsigned for i64 {
    type Unsigned = u64;
    fn into_u(&self) -> Self::Unsigned {
        *self as u64
    }
}
