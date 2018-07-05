mod error;

pub use self::error::*;
use alloc::boxed::Box;
use core::iter::Iterator;
use core::result::Result;
use serde::de::Visitor;
use serde::Deserializer;

// pub struct EosDeserializer<'a, R>
// where
//     R: Iterator<Item = &'a u8>,
// {
//     bytes: R,
// }

// impl<'a, R> EosDeserializer<'a, R>
// where
//     R: Iterator<Item = &'a u8>,
// {
//     pub fn new(bytes: &[u8]) -> Self {
//         EosDeserializer {
//             bytes: bytes.into_iter(),
//         }
//     }
// }

pub struct EosDeserializer<'a> {
    bytes: ::core::slice::Iter<'a, u8>,
}

impl<'a> EosDeserializer<'a> {
    pub fn new(bytes: &'a [u8]) -> EosDeserializer<'a> {
        EosDeserializer {
            bytes: bytes.into_iter(),
        }
    }
}

macro_rules! not_supported {
    ($method_name: ident) => {
        fn $method_name<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de> {
                Err(Box::new(ErrorKind::DeserializeNotSupported))
            }
    };
}

impl<'a, 'de> Deserializer<'de> for &'a mut EosDeserializer<'a> {
    type Error = Error;

    not_supported!(deserialize_any);
    not_supported!(deserialize_bool);
    not_supported!(deserialize_i8);
    not_supported!(deserialize_i16);
    not_supported!(deserialize_i32);
    not_supported!(deserialize_i64);
    not_supported!(deserialize_u8);
    not_supported!(deserialize_u16);
    not_supported!(deserialize_u32);
    not_supported!(deserialize_u64);
    not_supported!(deserialize_f32);
    not_supported!(deserialize_f64);
    not_supported!(deserialize_char);
    not_supported!(deserialize_str);
    not_supported!(deserialize_string);
    not_supported!(deserialize_bytes);
    not_supported!(deserialize_byte_buf);
    not_supported!(deserialize_option);
    not_supported!(deserialize_unit);
    not_supported!(deserialize_seq);
    not_supported!(deserialize_map);
    not_supported!(deserialize_identifier);
    not_supported!(deserialize_ignored_any);

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Box::new(ErrorKind::DeserializeNotSupported))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Box::new(ErrorKind::DeserializeNotSupported))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Box::new(ErrorKind::DeserializeNotSupported))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Box::new(ErrorKind::DeserializeNotSupported))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Box::new(ErrorKind::DeserializeNotSupported))
    }
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Box::new(ErrorKind::DeserializeNotSupported))
    }
}
