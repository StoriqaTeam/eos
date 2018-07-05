use alloc::string::String;
use error::Error;

pub trait Deserialize: Sized {
    fn deserialize(d: Reader) -> Result<Self, Error>;
}

pub struct Reader<'a> {
    bytes: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new<'b>(bytes: &'b [u8]) -> Reader<'b> {
        Reader { bytes }
    }

    pub fn read_sized<T>(&mut self) -> Result<T, Error> {
        let size = ::core::mem::size_of::<T>();
        if size > self.bytes.len() {
            return Err(Error::MemoryOutOfBounds)
        }
        let ptr: *const T = self.bytes.as_ptr() as *const T;
        self.bytes = &self.bytes[size..];
        unsafe {
            Ok(ptr.read())
        }
    }

    pub fn read_bytes(&mut self) -> Result<&[u8], Error> {
        let l_byte: u8 = self.read_sized()?;
        let len: usize = if l_byte <= 128 {
            l_byte as usize
        } else {
            let s_byte: u8 = self.read_sized()?;
            (s_byte as usize) >> 7 + (l_byte as usize - 128)
        };
        if len > self.bytes.len() {
            return Err(Error::MemoryOutOfBounds)
        }
        let bytes = &self.bytes[..len];
        self.bytes = &self.bytes[len..];
        Ok(bytes)
    }

    pub fn read_string(&mut self) -> Result<String, Error> {
        let bytes = self.read_bytes()?;
        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => Ok(s),
            Err(_) => Err(Error::Utf8Error),
        }
    }
}
