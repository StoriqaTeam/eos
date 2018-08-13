//! Custom memory allocator

use alloc::alloc::Layout;
use core::alloc::GlobalAlloc;

use types::Opaque;

extern "C" {
    // #[link_name = "llvm.wasm.current.memory.i32"]
    // fn current_memory() -> usize;

    // #[link_name = "llvm.wasm.grow.memory.i32"]
    // fn grow_memory(pages: usize) -> i32;
}

/// Start address of memory for smart contract
/// Strange behaviour: If put 0 here wasm generates unreachable instruction!
pub const START_ADDRESS: *mut u16 = 2 as *mut u16;

/// Custom Allocator implementation
#[derive(Debug, Copy, Clone)]
pub struct Allocator;

impl Allocator {
    /// Init for Allocator, needs to set start address offset of 2 bytes.
    /// This is an ugly workaround. TODO: Explore this issue
    pub fn init() {
        unsafe {
            // grow_memory(1);
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

    /// TODO: Check the up-to-date approach to extern pointers. I think they are using *u8 now or smth like that
    #[inline]
    unsafe fn dealloc(&self, _ptr: *mut Opaque, _layout: Layout) {}
}
