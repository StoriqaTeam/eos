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
    eos::print_str("Deployed");
}

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    allocator::Allocator::init();
    // eos::print_u64(action);
    // eos::print_str("\n");
    // eos::print_u64(eos::str_to_name("review.upd"));
    // eos::print_str("\n");
    if action == eos::str_to_name("review.add") {
        if let Ok(review) = eos::read_action::<Review>() {
            review_add(receiver, review);
        } else {
            eos::print_str("Failed to deserialize data for `review.add` action\n");
        }
    } else if action == eos::str_to_name("review.read") {
        if let Ok(ReadReviewAction { id }) = eos::read_action::<ReadReviewAction>() {
            review_read(receiver, id);
        } else {
            eos::print_str("Failed to deserialize data for `review.read` action\n");
        }
    } else if action == eos::str_to_name("review.upd") {
        if let Ok(review) = eos::read_action::<Review>() {
            review_update(receiver, review);
        } else {
            eos::print_str("Failed to deserialize data for `review.upd` action\n");
        }
    } else {
        eos::print_str("No such action\n");
    }
}

const TABLE_NAME: u64 = 1;

fn review_add(receiver: u64, review: Review) {
    eos::print_str("Received action `review.add` for id: ");
    eos::print_u64(review.id);
    eos::print_str("\n");
    eos::db_store(receiver, TABLE_NAME, receiver, review.id, &review);
}

fn review_update(receiver: u64, mut review: Review) {
    eos::print_str("Received action `review.update` for id: ");
    eos::print_u64(review.id);
    eos::print_str("\n");
    eos::db_update(receiver, receiver, receiver, TABLE_NAME, review.id, &mut review);
}


fn review_read(receiver: u64, id: u64) {
    eos::print_str("Received action `review.read` for id: ");
    eos::print_u64(id);
    eos::print_str("\n");
    if let Ok(review) = eos::db_read::<Review>(receiver, receiver, TABLE_NAME, id) {
        eos::print_str("Found review with id: ");
        eos::print_u64(review.id);
        eos::print_str("\n");
        eos::print_str("byte1: ");
        eos::print_u64(review.byte1);
        eos::print_str("\n");
        eos::print_str("byte2: ");
        eos::print_u64(review.byte2);
        eos::print_str("\n");
        eos::print_str("byte3: ");
        eos::print_u64(review.byte3);
        eos::print_str("\n");
        eos::print_str("byte4: ");
        eos::print_u64(review.byte4);
        eos::print_str("\n");
    } else {
        eos::print_str("Unable to read data from db\n");
    }

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
