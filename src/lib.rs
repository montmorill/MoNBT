#![allow(non_upper_case_globals)]

use arbitrary::{Arbitrary, Result, Unstructured};

use crate::mutf8::{Mutf8Str, Mutf8String};

pub mod mutf8;
pub mod owned;
pub mod tag;

pub trait Deserialize<'a, T> {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<T>;
}

macro_rules! deserialize_impl {
    ($edition:ident $method:ident $($ty:ty)*) => {
        $(
            impl<'a> Deserialize<'a, $ty> for $edition {
                fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<$ty> {
                    Ok(<$ty>::$method(u.arbitrary()?))
                }
            }
        )*
    };
}

#[derive(Debug, Default)]
pub struct JavaEdition;

deserialize_impl!(JavaEdition from_be_bytes i8 i16 i32 i64 u16 f32 f64);

#[derive(Debug, Default)]
pub struct BedrockEdition;
deserialize_impl!(BedrockEdition from_le_bytes i8 i16 i32 i64 u16 f32 f64);

#[derive(Debug, Default)]
pub struct VarIntEdition;
deserialize_impl!(VarIntEdition from_le_bytes i8 i16 u16 f32 f64);

impl<'a> Deserialize<'a, i32> for VarIntEdition {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<i32> {
        let mut value = 0;
        for size in 0..5 {
            let byte = u8::arbitrary(u)?;
            value |= ((byte & 0b0111_1111) as i32) << (size * 7);
            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }
        Ok((value >> 1) ^ (-(value & 1)))
    }
}

impl<'a> Deserialize<'a, i64> for VarIntEdition {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<i64> {
        let mut value = 0;
        for size in 0..10 {
            let byte = u8::arbitrary(u)?;
            value |= ((byte & 0b0111_1111) as i64) << (size * 7);
            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }
        Ok((value >> 1) ^ (-(value & 1)))
    }
}

impl<'a, T: Deserialize<'a, u16>> Deserialize<'a, &'a Mutf8Str> for T {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<&'a Mutf8Str> {
        let len: u16 = self.deserialize(u)?;
        Ok(Mutf8Str::from_slice(u.bytes(len as usize)?))
    }
}

impl<'a, T: Deserialize<'a, &'a Mutf8Str>> Deserialize<'a, Mutf8String> for T {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<Mutf8String> {
        self.deserialize(u).map(ToOwned::to_owned)
    }
}

pub trait DeserializeTag<'a>:
    Deserialize<'a, u16>
    + Deserialize<'a, i8>
    + Deserialize<'a, i16>
    + Deserialize<'a, i32>
    + Deserialize<'a, i64>
    + Deserialize<'a, f32>
    + Deserialize<'a, f64>
    // + Deserialize<'a, &'a Mutf8Str>
    // + Deserialize<'a, Mutf8String>
{
}

impl<'a, T> DeserializeTag<'a> for T where
    T: Deserialize<'a, u16>
        + Deserialize<'a, i8>
        + Deserialize<'a, i16>
        + Deserialize<'a, i32>
        + Deserialize<'a, i64>
        + Deserialize<'a, f32>
        + Deserialize<'a, f64>
{
}
