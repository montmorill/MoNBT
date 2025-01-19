use std::collections::HashMap;

use arbitrary::{Arbitrary, Error, Result, Unstructured};

use crate::mutf8::Mutf8String;
use crate::{tag::Tag, Deserialize, Edition};

#[derive(Debug, Clone)]
pub enum OwnedPayload<const Java: bool, const Variant: bool> {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(Mutf8String),
    List(OwnedList<Java, Variant>),
    Compound(OwnedCompound<Java, Variant>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Debug, Clone)]
pub enum OwnedList<const Java: bool, const Variant: bool> {
    Byte(Vec<i8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<Vec<i8>>),
    String(Vec<Mutf8String>),
    List(Vec<OwnedList<Java, Variant>>),
    Compound(Vec<OwnedCompound<Java, Variant>>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
}

pub type OwnedCompound<const Java: bool, const Variant: bool> =
    HashMap<Mutf8String, OwnedPayload<Java, Variant>>;
pub type OwnedNamedTag<const Java: bool, const Variant: bool> =
    (Mutf8String, OwnedPayload<Java, Variant>);

impl<'a, T, const Java: bool, const Variant: bool> Deserialize<'a, T> for Tag<Java, Variant>
where
    Edition<Java, Variant>: Deserialize<'a, T>,
{
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<T> {
        Edition::<Java, Variant>.deserialize(u)
    }
}

impl<'a, T, const Java: bool, const Variant: bool> Deserialize<'a, Vec<T>>
    for Edition<Java, Variant>
where
    Self: Deserialize<'a, T>,
{
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<Vec<T>> {
        let len = <Self as Deserialize<'a, i32>>::deserialize(self, u)?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(self.deserialize(u)?);
        }
        Ok(vec)
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, OwnedPayload<Java, Variant>>
    for Tag<Java, Variant>
{
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedPayload<Java, Variant>> {
        Ok(match self {
            Tag::End => return Err(Error::IncorrectFormat),
            Tag::Byte => OwnedPayload::Byte(self.deserialize(u)?),
            Tag::Short => OwnedPayload::Short(self.deserialize(u)?),
            Tag::Int => OwnedPayload::Int(self.deserialize(u)?),
            Tag::Long => OwnedPayload::Long(self.deserialize(u)?),
            Tag::Float => OwnedPayload::Float(self.deserialize(u)?),
            Tag::Double => OwnedPayload::Double(self.deserialize(u)?),
            Tag::ByteArray => OwnedPayload::ByteArray(self.deserialize(u)?),
            Tag::String => OwnedPayload::String(self.deserialize(u)?),
            Tag::List => OwnedPayload::List(self.deserialize(u)?),
            Tag::Compound => OwnedPayload::Compound(self.deserialize(u)?),
            Tag::IntArray => OwnedPayload::IntArray(self.deserialize(u)?),
            Tag::LongArray => OwnedPayload::LongArray(self.deserialize(u)?),
        })
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, OwnedList<Java, Variant>>
    for Edition<Java, Variant>
{
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedList<Java, Variant>> {
        Ok(match Tag::<Java, Variant>::arbitrary(u)? {
            Tag::End => return Err(Error::IncorrectFormat),
            Tag::Byte => OwnedList::Byte(self.deserialize(u)?),
            Tag::Short => OwnedList::Short(self.deserialize(u)?),
            Tag::Int => OwnedList::Int(self.deserialize(u)?),
            Tag::Long => OwnedList::Long(self.deserialize(u)?),
            Tag::Float => OwnedList::Float(self.deserialize(u)?),
            Tag::Double => OwnedList::Double(self.deserialize(u)?),
            Tag::ByteArray => OwnedList::ByteArray(self.deserialize(u)?),
            Tag::String => OwnedList::String(self.deserialize(u)?),
            Tag::List => OwnedList::List(self.deserialize(u)?),
            Tag::Compound => OwnedList::Compound(self.deserialize(u)?),
            Tag::IntArray => OwnedList::IntArray(self.deserialize(u)?),
            Tag::LongArray => OwnedList::LongArray(self.deserialize(u)?),
        })
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, OwnedCompound<Java, Variant>>
    for Edition<Java, Variant>
{
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedCompound<Java, Variant>> {
        let mut map = HashMap::new();
        loop {
            match Tag::arbitrary(u)? {
                Tag::End => break Ok(map),
                tag => {
                    let name = self.deserialize(u)?;
                    let payload = tag.deserialize(u)?;
                    map.insert(name, payload);
                }
            }
        }
    }
}

impl<'a, const Java: bool, const Variant: bool> Deserialize<'a, OwnedNamedTag<Java, Variant>>
    for Edition<Java, Variant>
{
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedNamedTag<Java, Variant>> {
        let tag = Tag::arbitrary(u)?;
        let name = self.deserialize(u)?;
        let payload = tag.deserialize(u)?;
        Ok((name, payload))
    }
}
