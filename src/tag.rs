use std::marker::PhantomData;

use arbitrary::{Arbitrary, Error, Result, Unstructured};

use crate::Edition;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Tag<T: Edition> {
    End(PhantomData<T>),
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

impl<'a, T: Edition> Arbitrary<'a> for Tag<T> {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(match u8::arbitrary(u)? {
            0 => Tag::End(PhantomData),
            1 => Tag::Byte,
            2 => Tag::Short,
            3 => Tag::Int,
            4 => Tag::Long,
            5 => Tag::Float,
            6 => Tag::Double,
            7 => Tag::ByteArray,
            8 => Tag::String,
            9 => Tag::List,
            10 => Tag::Compound,
            11 => Tag::IntArray,
            12 => Tag::LongArray,
            _ => return Err(Error::IncorrectFormat),
        })
    }
}
