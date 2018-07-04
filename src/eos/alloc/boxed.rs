use core::ops::Deref;

use super::unique::Unique;
use super::super::{free, malloc, Void};


pub struct Box<T> {
    ptr: Unique<T>
}

impl<T> Box<T> {
    pub unsafe fn from_raw(raw_ptr: *mut T) -> Self {
        Box {ptr: Unique::new(raw_ptr)}
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr.as_ptr() as *mut Void);
        }
    }
}

impl<T> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            &*self.ptr.as_ptr()
        }
    }
}
