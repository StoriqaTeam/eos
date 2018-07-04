#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(alloc)]
#![feature(global_alloc)]
#![feature(oom)]
#![no_std]

extern crate alloc;

mod eos;
mod allocator;

use core::ops::Deref;
use core::panic::PanicInfo;
use alloc::boxed::Box;

#[global_allocator]
pub static ALLOC: allocator::Allocator = allocator::Allocator;

#[repr(C)]
struct HiAction {
    pub name: u64,
}

#[no_mangle]
pub extern "C" fn init() {
    eos::print_str("Yo - deployed");
}

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    allocator::Allocator::init();
    // let b = Box::new(receiver + code);
    //  eos::print_u64(*b);

    // eos::print_str("Receiver: ");
    // let b: Box<i64> = Box::new(5);
    // eos::print_i64(*b);
    // let mem_top: *mut u16 = allocator::START_ADDRESS;
    // let p = 0x0u64 as *mut u64;
    // unsafe {
    //     p.write(123);
    //     let x: u64 = p.read();
    //     // eos::print_u64(x);
    // }

    // eos::print_name(receiver);
    // eos::print_str(" Code: ");
    // eos::print_name(code);
    // eos::print_str(" Action: ");
    // eos::print_name(action);
    // unsafe {
    //     eos::print_u64(eos::action_data_size() as u64);
    // }
    let act = eos::read_action::<HiAction>();
    eos::print_str(" Read Action: ");
    eos::print_name(act.deref().name);

    // if action == eos::str_to_name("hi") {
    //     hi();
    // } else {
    //     eos::print_str("No such action");
    // }
}

// fn hi() {
//     eos::print_str("Received action HI!");
// }

/// This function is called on panic.
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Need to provide a tiny `oom` lang-item implementation for
// `#![no_std]`.
#[lang = "oom"]
#[no_mangle]
pub extern "C" fn oom() -> ! {
    loop {}
}
