#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(alloc)]
#![feature(global_alloc)]
#![feature(oom)]
#![feature(core_intrinsics)]
#![no_std]

#[macro_use]
extern crate alloc;

mod allocator;
mod eos;
mod models;

use core::intrinsics::abort;
use core::panic::PanicInfo;
use models::*;

#[global_allocator]
pub static ALLOC: allocator::Allocator = allocator::Allocator;

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
        eos::print_str(&data.message);
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
    eos::print_str("Wasm panicked!");
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
