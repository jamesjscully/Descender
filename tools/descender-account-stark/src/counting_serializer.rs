//! A `serde::Serializer` that counts bytes instead of producing output, giving a
//! compact-binary (bincode-like) serialized size for the STARK proof without
//! pulling in a bincode dependency.
//!
//! Encoding model (matches bincode's default fixint-ish layout closely enough for
//! a size estimate): primitives = their byte width; bool = 1; seq/map length =
//! 8-byte prefix; str/bytes = 8-byte length prefix + payload; enum variant index
//! = 4 bytes; Option = 1 tag byte (+ payload). Field elements serialize as their
//! canonical integer (u32 for 31-bit fields, u64 for Goldilocks), so the count
//! tracks real proof size up to constant-factor framing.

use serde::ser::{
    Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

pub struct ByteCounter(pub usize);

#[derive(Debug)]
pub struct CountErr;
impl core::fmt::Display for CountErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "byte-counter serializer error")
    }
}
impl std::error::Error for CountErr {}
impl serde::ser::Error for CountErr {
    fn custom<T: core::fmt::Display>(_: T) -> Self {
        CountErr
    }
}

impl ByteCounter {
    fn add(&mut self, n: usize) {
        self.0 += n;
    }
}

impl<'a> Serializer for &'a mut ByteCounter {
    type Ok = ();
    type Error = CountErr;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _: bool) -> Result<(), CountErr> {
        self.add(1);
        Ok(())
    }
    fn serialize_i8(self, _: i8) -> Result<(), CountErr> {
        self.add(1);
        Ok(())
    }
    fn serialize_i16(self, _: i16) -> Result<(), CountErr> {
        self.add(2);
        Ok(())
    }
    fn serialize_i32(self, _: i32) -> Result<(), CountErr> {
        self.add(4);
        Ok(())
    }
    fn serialize_i64(self, _: i64) -> Result<(), CountErr> {
        self.add(8);
        Ok(())
    }
    fn serialize_i128(self, _: i128) -> Result<(), CountErr> {
        self.add(16);
        Ok(())
    }
    fn serialize_u8(self, _: u8) -> Result<(), CountErr> {
        self.add(1);
        Ok(())
    }
    fn serialize_u16(self, _: u16) -> Result<(), CountErr> {
        self.add(2);
        Ok(())
    }
    fn serialize_u32(self, _: u32) -> Result<(), CountErr> {
        self.add(4);
        Ok(())
    }
    fn serialize_u64(self, _: u64) -> Result<(), CountErr> {
        self.add(8);
        Ok(())
    }
    fn serialize_u128(self, _: u128) -> Result<(), CountErr> {
        self.add(16);
        Ok(())
    }
    fn serialize_f32(self, _: f32) -> Result<(), CountErr> {
        Err(CountErr)
    }
    fn serialize_f64(self, _: f64) -> Result<(), CountErr> {
        Err(CountErr)
    }
    fn serialize_char(self, _: char) -> Result<(), CountErr> {
        self.add(4);
        Ok(())
    }
    fn serialize_str(self, v: &str) -> Result<(), CountErr> {
        self.add(8 + v.len());
        Ok(())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<(), CountErr> {
        self.add(8 + v.len());
        Ok(())
    }
    fn serialize_none(self) -> Result<(), CountErr> {
        self.add(1);
        Ok(())
    }
    fn serialize_some<T: ?Sized + Serialize>(self, v: &T) -> Result<(), CountErr> {
        self.add(1);
        v.serialize(self)
    }
    fn serialize_unit(self) -> Result<(), CountErr> {
        Ok(())
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), CountErr> {
        Ok(())
    }
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<(), CountErr> {
        self.add(4);
        Ok(())
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        v: &T,
    ) -> Result<(), CountErr> {
        v.serialize(self)
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        v: &T,
    ) -> Result<(), CountErr> {
        self.add(4);
        v.serialize(self)
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, CountErr> {
        self.add(8); // length prefix
        Ok(self)
    }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, CountErr> {
        Ok(self)
    }
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, CountErr> {
        Ok(self)
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, CountErr> {
        self.add(4);
        Ok(self)
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, CountErr> {
        self.add(8);
        Ok(self)
    }
    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, CountErr> {
        Ok(self)
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, CountErr> {
        self.add(4);
        Ok(self)
    }
}

macro_rules! impl_compound {
    ($trait:ident, $method:ident) => {
        impl<'a> $trait for &'a mut ByteCounter {
            type Ok = ();
            type Error = CountErr;
            fn $method<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), CountErr> {
                value.serialize(&mut **self)
            }
            fn end(self) -> Result<(), CountErr> {
                Ok(())
            }
        }
    };
}

impl_compound!(SerializeSeq, serialize_element);
impl_compound!(SerializeTuple, serialize_element);
impl_compound!(SerializeTupleStruct, serialize_field);
impl_compound!(SerializeTupleVariant, serialize_field);

impl<'a> SerializeMap for &'a mut ByteCounter {
    type Ok = ();
    type Error = CountErr;
    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), CountErr> {
        key.serialize(&mut **self)
    }
    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), CountErr> {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), CountErr> {
        Ok(())
    }
}

impl<'a> SerializeStruct for &'a mut ByteCounter {
    type Ok = ();
    type Error = CountErr;
    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _: &'static str,
        value: &T,
    ) -> Result<(), CountErr> {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), CountErr> {
        Ok(())
    }
}

impl<'a> SerializeStructVariant for &'a mut ByteCounter {
    type Ok = ();
    type Error = CountErr;
    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _: &'static str,
        value: &T,
    ) -> Result<(), CountErr> {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), CountErr> {
        Ok(())
    }
}
