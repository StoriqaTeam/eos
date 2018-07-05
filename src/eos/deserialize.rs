pub trait Deserialize: Sized {
    type Error;
    fn deserialize(d: Reader) -> Result<Self, Self::Error>;
}

pub struct Reader {
    ptr: *const u8,
}

impl Reader {
    pub fn new(ptr: *const u8) -> Self {
        Reader { ptr }
    }

    pub fn read<T>(&mut self) -> T {
        unsafe {
            let res = (self.ptr as *const T).read();
            self.ptr = self.ptr.offset(::core::mem::size_of::<T>() as isize);
            res
        }
    }
}
