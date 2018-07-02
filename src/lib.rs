#![feature(lang_items)]
#![feature(panic_implementation)]
#![no_std]

mod eos;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn init() {
    eos::print_str("Yo - deployed");
}

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    eos::print_str("Receiver: ");
    eos::print_name(receiver);
    eos::print_str(" Code: ");
    eos::print_name(code);
    eos::print_str(" Action: ");
    eos::print_name(action);

    if action == eos::str_to_name("hi") {
        hi();
    } else {
        eos::print_str("No such action");
    }
}

fn hi() {
    eos::print_str("Received action HI!");
}

/// This function is called on panic.
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
