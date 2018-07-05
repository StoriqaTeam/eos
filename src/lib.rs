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
mod error;

use alloc::string::String;
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
    if action == eos::str_to_name("review") {
        let ReviewAction { user, hash, mark } = eos::read_action::<ReviewAction>();
        review(user, hash, mark);
    } else {
        eos::print_str("No such action");
    }
}

fn review(user: u64, hash: String, mark: i32) {
    eos::print_str("Received action review for user: ");
    eos::print_name(user);
    eos::print_str(" hash: ");
    eos::print_str(&hash);
    eos::print_str(" mark: ");
    eos::print_i64(mark as i64);
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
