#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(alloc)]
#![feature(global_alloc)]
#![feature(oom)]
#![feature(core_intrinsics)]
#![no_std]

#[macro_use]
extern crate alloc;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod allocator;
mod eos;

use core::intrinsics::abort;
use core::panic::PanicInfo;

#[global_allocator]
pub static ALLOC: allocator::Allocator = allocator::Allocator;

#[derive(Deserialize)]
#[repr(C)]
struct HiAction {
    // pub name: u64,
    pub byte1: u8,
    pub byte2: u8,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub byte7: u8,
    pub byte8: u8,
    pub byte9: u8,
    pub byte10: u8,
    pub byte11: u8,
    pub byte12: u8,
    pub byte13: u8,
    pub byte14: u8,
}

#[no_mangle]
pub extern "C" fn init() {
    eos::print_str("Yo - deployed");
}

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    allocator::Allocator::init();
    eos::print_str("Receiver: ");
    eos::print_name(receiver);
    eos::print_str(" Code: ");
    eos::print_name(code);
    eos::print_str(" Action: ");
    eos::print_name(action);
    eos::print_str(" ");

    if action == eos::str_to_name("hi") {
        let data = eos::read_action::<HiAction>();
        eos::print_u64(data.byte1 as u64);
        eos::print_str(" ");
        eos::print_u64(data.byte2 as u64);
        eos::print_str(" ");
        eos::print_u64(data.byte3 as u64);
        eos::print_str(" ");
        eos::print_u64(data.byte4 as u64);
        eos::print_str(" ");
        eos::print_u64(data.byte5 as u64);
        eos::print_str(" ");
        eos::print_u64(data.byte6 as u64);
        eos::print_str(" ");
        eos::print_u64(data.byte7 as u64);
        eos::print_str(" ");
        eos::print_u64(data.byte8 as u64);
        eos::print_str(" ");
        eos::print_u64(data.byte9 as u64);
        eos::print_str(" ");
        eos::print_u64(data.byte10 as u64);
        eos::print_str(" ");
        eos::print_u64(data.byte11 as u64);
        eos::print_str(" ");
        eos::print_u64(data.byte12 as u64);
        eos::print_str(" ");

    // hi(data.name);
    } else {
        eos::print_str("No such action");
    }
}

fn hi(name: u64) {
    eos::print_str("Received action Hi for name: ");
    eos::print_name(name);
}

/// This function is called on panic.
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    eos::print_str("Wasm paniced!");
    unsafe { abort() }
}

// Need to provide a tiny `oom` lang-item implementation for
// `#![no_std]`.
#[lang = "oom"]
#[no_mangle]
pub extern "C" fn oom() -> ! {
    eos::print_str("Out of memory!");
    unsafe { abort() }
}
