mod error;

pub use self::error::Error;

pub trait Deserialize: Sized {
    fn deserialize(mut d: Deserializer) -> Result<Self, Error>;
}

pub struct Deserializer {
    // bytes: ::core::slice::Iter<'a, u8>,
}

impl Deserializer {
    pub fn new(bytes: &[u8]) -> Deserializer {
        Deserializer {}
        // bytes: bytes.into_iter(),
    }

    pub fn read_u8(&mut self) -> Result<u8, Error> {
        loop {}
        // self.bytes.next().cloned().ok_or(Error::EOF)
    }
}
