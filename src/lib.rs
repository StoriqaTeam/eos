//! EOS smart contract implementation.
//!
//! Rust implementation of smart contracts, provides a few major components:
//!
//! * Bindings to WASM functions.
//! * Examples of smart contracts.
//!
//! Library uses unsafe FFI bindings and works currently only on nightly Rust.
//!
//! EOS developers documentation is found on the [website].
//!
//! [website]: https://developers.eos.io/eosio-cpp/reference.
//!
//! # Examples
//!
//! A simple smart contract:
//!
//! ```no_run
//! extern crate eos;
//!
//! fn review_add(receiver: u64, review: Review) {
//!     eos::print_str("Received action `review.add` for id: ");
//!     eos::print_u64(review.id);
//!     eos::print_str("\n");
//!     eos::db_store(receiver, TABLE_NAME, receiver, review.id, &review);
//! }
//! ```

#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(alloc)]
#![feature(global_alloc)]
#![feature(oom)]
#![feature(core_intrinsics)]
#![no_std]
#![deny(
    missing_docs,
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

//#[macro_use]
extern crate alloc;

mod allocator;
pub mod eos;
mod error;
mod models;

pub use allocator::*;
pub use eos::console::*;

use alloc::alloc::Layout;
use core::intrinsics::abort;
use core::panic::PanicInfo;

/// Custom EOS allocator
#[global_allocator]
pub static ALLOC: Allocator = Allocator;

/// This function is needed for global allocator.
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// This function is needed for global allocator.
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    print_str("Wasm panicked!");
    unsafe { abort() }
}

/// This function is needed for global allocator.
#[lang = "oom"]
#[no_mangle]
pub extern "C" fn oom(_: Layout) -> ! {
    print_str("Out of memory!");
    unsafe { abort() }
}
