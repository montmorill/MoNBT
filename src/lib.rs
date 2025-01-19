#![allow(non_upper_case_globals)]

use arbitrary::{Arbitrary, Result, Unstructured};

use crate::mutf8::{Mutf8Str, Mutf8String};

pub mod mutf8;
pub mod owned;
mod tag;

pub struct Edition<const Java: bool, const Variant: bool>;
pub type JavaEdition<const Network: bool = false> = Edition<true, Network>;
pub type JavaEditionNetwork = JavaEdition<true>;

pub type BedrockEdition<const VarInt: bool = false> = Edition<false, VarInt>;
pub type BedrockEditionVarInt = BedrockEdition<true>;

pub trait Deserialize<'a, T> {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<T>;
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, &'a Mutf8Str>
    for Edition<Java, Variant>
{
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<&'a Mutf8Str> {
        let len: u16 = self.deserialize(u)?;
        Ok(Mutf8Str::from_slice(u.bytes(len as usize)?))
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, Mutf8String>
    for Edition<Java, Variant>
{
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<Mutf8String> {
        self.deserialize(u).map(ToOwned::to_owned)
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, i8> for Edition<Java, Variant> {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<i8> {
        u.arbitrary()
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, i16> for Edition<Java, Variant> {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<i16> {
        Ok(if Java {
            i16::from_be_bytes
        } else {
            i16::from_le_bytes
        }(u.arbitrary()?))
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, i32> for Edition<Java, Variant> {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<i32> {
        Ok(match (Java, Variant) {
            (true, _) => i32::from_be_bytes(u.arbitrary()?),
            (false, false) => i32::from_le_bytes(u.arbitrary()?),
            (false, true) => {
                let mut value = 0;
                for size in 0..5 {
                    let byte = u8::arbitrary(u)?;
                    value |= ((byte & 0b0111_1111) as i32) << (size * 7);
                    if (byte & 0b1000_0000) == 0 {
                        break;
                    }
                }
                (value >> 1) ^ (-(value & 1))
            }
        })
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, i64> for Edition<Java, Variant> {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<i64> {
        Ok(match (Java, Variant) {
            (true, _) => i64::from_be_bytes(u.arbitrary()?),
            (false, false) => i64::from_le_bytes(u.arbitrary()?),
            (false, true) => {
                let mut value = 0;
                for size in 0..10 {
                    let byte = u8::arbitrary(u)?;
                    value |= ((byte & 0b0111_1111) as i64) << (size * 7);
                    if (byte & 0b1000_0000) == 0 {
                        break;
                    }
                }
                (value >> 1) ^ (-(value & 1))
            }
        })
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, f32> for Edition<Java, Variant> {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<f32> {
        Ok(if Java {
            f32::from_be_bytes
        } else {
            f32::from_le_bytes
        }(u.arbitrary()?))
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, f64> for Edition<Java, Variant> {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<f64> {
        Ok(if Java {
            f64::from_be_bytes
        } else {
            f64::from_le_bytes
        }(u.arbitrary()?))
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, u16> for Edition<Java, Variant> {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<u16> {
        Ok(if Java {
            u16::from_be_bytes
        } else {
            u16::from_le_bytes
        }(u.arbitrary()?))
    }
}
