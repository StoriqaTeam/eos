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
mod error;
mod models;

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
        if let Ok(ReviewAction { user, hash, mark }) = eos::read_action::<ReviewAction>() {
            review(receiver, user, hash, mark);
        } else {
            eos::print_str("Failed to deserialize data for `review` action");
        }
    } else if action == eos::str_to_name("read") {
        if let Ok(ReadReviewAction { user }) = eos::read_action::<ReadReviewAction>() {
            read(receiver, user);
        } else {
            eos::print_str("Failed to deserialize data for `read` action");
        }
    } else {
        eos::print_str("No such action");
    }
}

const TABLE_NAME: u64 = 1;

fn review(receiver: u64, user: u64, hash: String, mark: i32) {
    eos::print_str("Received action `review` for user: ");
    eos::print_name(user);
    eos::print_str("with number: ");
    eos::print_u64(user);

    eos::print_str(" hash: ");
    eos::print_str(&hash);
    eos::print_str(" mark: ");
    eos::print_i64(mark as i64);
    eos::store_bytes(receiver, TABLE_NAME, receiver, user, hash.as_bytes())
}

fn read(receiver: u64, user: u64) {
    eos::print_str("Received action `read` for a user: ");
    eos::print_name(user);
    eos::print_str(" ");
    let bytes = eos::read_bytes(receiver, receiver, TABLE_NAME, user);
    match String::from_utf8(bytes) {
        Ok(msg) => {
            eos::print_str("Deserialized message: ");
            eos::print_str(&msg);
        }
        Err(e) => eos::print_str("Error deserializing bytes"),
    };
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
