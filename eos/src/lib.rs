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

#![no_std]
#![feature(alloc, global_alloc, core_intrinsics, oom, panic_implementation, lang_items)]
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

extern crate alloc;
extern crate wee_alloc;
#[macro_use]
extern crate cfg_if;

mod allocators;
pub mod bindings;
pub mod deserialize;
pub mod error;
pub mod types;

cfg_if! {
    if #[cfg(feature = "custom_allocator")] {
        /// Custom EOS allocator
        #[global_allocator]
        pub static GLOBAL_ALLOCATOR: allocators::custom::Allocator = allocators::custom::Allocator;
    } else if #[cfg(feature = "wee_allocator")] {
        /// Wee allocator
        #[global_allocator]
        pub static GLOBAL_ALLOCATOR: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    } else {
        compile_error! {
            "There is no alloc specified!"
        }
    }
}

cfg_if! {
    if #[cfg(not(test))] {
        use alloc::alloc::Layout;
        use core::intrinsics::abort;
        use core::panic::PanicInfo;
        use bindings::console::*;

        /// This function is needed for global allocator with `#![no_std]`.
        #[lang = "eh_personality"]
        extern "C" fn eh_personality() {}

        /// This function is needed for global allocator with `#![no_std]`.
        #[panic_implementation]
        pub fn panic(_info: &PanicInfo) -> ! {
            print_str("Wasm panicked!");
            unsafe { abort() }
        }

        /// This function is needed for global allocator with `#![no_std]`.
        #[lang = "oom"]
        pub extern "C" fn oom(_: Layout) -> ! {
            print_str("Out of memory!");
            unsafe { abort() }
        }
    }
}
