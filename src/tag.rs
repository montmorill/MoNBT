use arbitrary::{Arbitrary, Error, Result, Unstructured};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Tag<const Java: bool, const Variant: bool> {
    End,
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

impl<'a, const Java: bool, const Variant: bool> Arbitrary<'a> for Tag<Java, Variant> {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(match u8::arbitrary(u)? {
            0 => Tag::End,
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
