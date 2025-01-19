use std::{collections::HashMap, marker::PhantomData};

use arbitrary::{Arbitrary, Result, Unstructured};

use crate::mutf8::Mutf8String;
use crate::Edition;
use crate::{tag::Tag, Deserialize};

#[derive(Debug, Clone)]
pub enum OwnedPayload<Edition> {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(Mutf8String),
    List(OwnedList<Edition>),
    Compound(OwnedCompound<Edition>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Debug, Clone)]
pub enum OwnedList<Edition> {
    Phantom(PhantomData<Edition>),
    Byte(Vec<i8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<Vec<i8>>),
    String(Vec<Mutf8String>),
    List(Vec<OwnedList<Edition>>),
    Compound(Vec<OwnedCompound<Edition>>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
}

pub type OwnedCompound<Edition> = HashMap<Mutf8String, OwnedPayload<Edition>>;
pub type OwnedNamedTag<Edition> = (Mutf8String, OwnedPayload<Edition>);

impl<'a, T: Edition, U> Deserialize<'a, Vec<U>> for T
where
    Self: Deserialize<'a, U>,
{
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<Vec<U>> {
        let len = <Self as Deserialize<'a, i32>>::deserialize(self, u)?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(self.deserialize(u)?);
        }
        Ok(vec)
    }
}

impl<'a, T: Edition> Deserialize<'a, OwnedPayload<T>> for Tag<T> {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedPayload<T>> {
        Ok(match self {
            Tag::End(_) => return Err(arbitrary::Error::IncorrectFormat),
            Tag::Byte => OwnedPayload::Byte(T::Instance.deserialize(u)?),
            Tag::Short => OwnedPayload::Short(T::Instance.deserialize(u)?),
            Tag::Int => OwnedPayload::Int(T::Instance.deserialize(u)?),
            Tag::Long => OwnedPayload::Long(T::Instance.deserialize(u)?),
            Tag::Float => OwnedPayload::Float(T::Instance.deserialize(u)?),
            Tag::Double => OwnedPayload::Double(T::Instance.deserialize(u)?),
            Tag::ByteArray => OwnedPayload::ByteArray(T::Instance.deserialize(u)?),
            Tag::String => OwnedPayload::String(T::Instance.deserialize(u)?),
            Tag::List => OwnedPayload::List(T::Instance.deserialize(u)?),
            Tag::Compound => OwnedPayload::Compound(T::Instance.deserialize(u)?),
            Tag::IntArray => OwnedPayload::IntArray(T::Instance.deserialize(u)?),
            Tag::LongArray => OwnedPayload::LongArray(T::Instance.deserialize(u)?),
        })
    }
}

impl<'a, T: Edition> Deserialize<'a, OwnedList<T>> for T {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedList<T>> {
        Ok(match Tag::<T>::arbitrary(u)? {
            Tag::End(_) => OwnedList::Phantom(PhantomData),
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

impl<'a, T: Edition> Deserialize<'a, OwnedCompound<T>> for T {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedCompound<T>> {
        let mut map = HashMap::new();
        loop {
            match Tag::arbitrary(u)? {
                Tag::End(_) => break Ok(map),
                tag => {
                    let name = self.deserialize(u)?;
                    let payload = tag.deserialize(u)?;
                    map.insert(name, payload);
                }
            }
        }
    }
}

impl<'a, T: Edition> Deserialize<'a, OwnedNamedTag<T>> for T {
    fn deserialize(&self, u: &mut Unstructured<'a>) -> Result<OwnedNamedTag<T>> {
        let tag = Tag::arbitrary(u)?;
        let name = self.deserialize(u)?;
        let payload = tag.deserialize(u)?;
        Ok((name, payload))
    }
}
