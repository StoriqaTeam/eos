use core::alloc::{Layout, GlobalAlloc};

type Opaque = u8;

// If I put 0 here wasm generates to unreachable instruction for some reason
pub const START_ADDRESS: *mut u16 = 2 as *mut u16;

pub struct Allocator;

impl Allocator {
    pub fn init() {
        unsafe {
            START_ADDRESS.write(START_ADDRESS as u16 + 2);
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut Opaque {
        let top = START_ADDRESS.read();
        START_ADDRESS.write(top + (layout.size() as u16));
        top as *mut Opaque
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut Opaque, layout: Layout) {}

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut Opaque {
        self.alloc(layout)
    }

    #[inline]
    unsafe fn realloc(
        &self,
        ptr: *mut Opaque,
        layout: Layout,
        new_size: usize
    ) -> *mut Opaque {
        // unimplemented
        loop {}
    }
}
