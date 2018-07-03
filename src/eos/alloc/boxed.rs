use super::unique::Unique;
use super::super::{free, malloc};

struct Box<T> {
    ptr: Unique<T>
}

impl<T> Box<T> {
    pub unsafe fn from_raw(raw_ptr: *mut T) -> Self {
        Box {ptr: Unique::new(raw_ptr)}
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        free(self.ptr.as_ptr());
    }
}
