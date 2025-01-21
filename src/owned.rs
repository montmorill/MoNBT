use std::{collections::HashMap, marker::PhantomData};

use arbitrary::{Arbitrary, Result, Unstructured};

use crate::mutf8::Mutf8String;
use crate::DeserializeTag;
use crate::{tag::Tag, Deserialize};

#[derive(Debug, Clone)]
pub enum OwnedPayload<T> {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(Mutf8String),
    List(OwnedList<T>),
    Compound(OwnedCompound<T>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Debug, Clone)]
pub enum OwnedList<T> {
    Phantom(PhantomData<T>),
    Byte(Vec<i8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<Vec<i8>>),
    String(Vec<Mutf8String>),
    List(Vec<OwnedList<T>>),
    Compound(Vec<OwnedCompound<T>>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
}

pub type OwnedCompound<T> = HashMap<Mutf8String, OwnedPayload<T>>;
pub type OwnedNamedTag<T> = (Mutf8String, OwnedPayload<T>);

impl<'a, T: Deserialize<'a, i32> + Deserialize<'a, U>, U> Deserialize<'a, Vec<U>> for T {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<Vec<U>> {
        let len: i32 = self.deserialize(u)?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(self.deserialize(u)?);
        }
        Ok(vec)
    }
}

impl<'a, T: DeserializeTag<'a> + Default> Deserialize<'a, OwnedPayload<T>> for Tag {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedPayload<T>> {
        Ok(match self {
            Tag::End => return Err(arbitrary::Error::IncorrectFormat),
            Tag::Byte => OwnedPayload::Byte(T::default().deserialize(u)?),
            Tag::Short => OwnedPayload::Short(T::default().deserialize(u)?),
            Tag::Int => OwnedPayload::Int(T::default().deserialize(u)?),
            Tag::Long => OwnedPayload::Long(T::default().deserialize(u)?),
            Tag::Float => OwnedPayload::Float(T::default().deserialize(u)?),
            Tag::Double => OwnedPayload::Double(T::default().deserialize(u)?),
            Tag::ByteArray => OwnedPayload::ByteArray(T::default().deserialize(u)?),
            Tag::String => OwnedPayload::String(T::default().deserialize(u)?),
            Tag::List => OwnedPayload::List(T::default().deserialize(u)?),
            Tag::Compound => OwnedPayload::Compound(T::default().deserialize(u)?),
            Tag::IntArray => OwnedPayload::IntArray(T::default().deserialize(u)?),
            Tag::LongArray => OwnedPayload::LongArray(T::default().deserialize(u)?),
        })
    }
}

impl<'a, T: DeserializeTag<'a> + Default> Deserialize<'a, OwnedList<T>> for T {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedList<T>> {
        Ok(match Tag::arbitrary(u)? {
            Tag::End => OwnedList::Phantom(PhantomData),
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

impl<'a, T: DeserializeTag<'a> + Default> Deserialize<'a, OwnedCompound<T>> for T {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedCompound<T>> {
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

impl<'a, T: DeserializeTag<'a> + Default> Deserialize<'a, OwnedNamedTag<T>> for T {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedNamedTag<T>> {
        let tag = Tag::arbitrary(u)?;
        let name = self.deserialize(u)?;
        let payload = tag.deserialize(u)?;
        Ok((name, payload))
    }
}
