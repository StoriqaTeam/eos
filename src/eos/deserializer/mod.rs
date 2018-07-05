mod error;

pub use self::error::Error;

pub trait Deserialize: Sized {
    fn deserialize(mut d: Deserializer) -> Result<Self, Error>;
}

pub struct Deserializer<'a> {
    bytes: ::core::slice::Iter<'a, u8>,
}

impl<'a> Deserializer<'a> {
    pub fn new<'b>(bytes: &'b [u8]) -> Deserializer<'b> {
        Deserializer {
            bytes: bytes.into_iter(),
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, Error> {
        self.bytes.next().cloned().ok_or(Error::EOF)
    }
}
