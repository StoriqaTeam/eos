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
    eos::print_u64(receiver);
    eos::print_str(" Code: ");
    eos::print_u64(code);
    eos::print_str(" Action: ");
    eos::print_u64(action);
}

/// This function is called on panic.
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
