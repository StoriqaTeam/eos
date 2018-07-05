use alloc::string::String;

pub trait Deserialize: Sized {
    type Error;
    fn deserialize(d: Reader) -> Result<Self, Self::Error>;
}

pub struct Reader {
    ptr: *mut u8,
}

impl Reader {
    pub fn new(ptr: *mut u8) -> Self {
        Reader { ptr }
    }

    pub fn read_primitive<T>(&mut self) -> T {
        unsafe {
            let res = (self.ptr as *const T).read();
            self.ptr = self.ptr.offset(::core::mem::size_of::<T>() as isize);
            res
        }
    }

    pub fn read_string(&mut self) -> String {
        let l_byte: u8 = self.read_primitive();
        let len: u16 = if l_byte <= 128 {
            l_byte as u16
        } else {
            let s_byte: u8 = self.read_primitive();
            (s_byte as u16) >> 7 + (l_byte as u16 - 128)
        };
        unsafe { String::from_raw_parts(self.ptr, len as usize, len as usize) }
    }
}
