#![feature(lang_items)]
#![feature(panic_implementation)]
#![no_std]

mod eos;

use core::ops::Deref;
use core::panic::PanicInfo;

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
    // eos::print_str("Receiver: ");
    // let b: Box<i64> = Box::new(5);
    // eos::print_i64(*b);
    unsafe {
        core::ptr::write(4u64 as *mut u64, 123);
        let x: u64 = core::ptr::read(4u64 as *const u64);
        eos::print_u64(x);
    }

    // eos::print_name(receiver);
    // eos::print_str(" Code: ");
    // eos::print_name(code);
    // eos::print_str(" Action: ");
    // eos::print_name(action);
    // unsafe {
    //     eos::print_u64(eos::action_data_size() as u64);
    // }
    // let act = eos::read_action::<HiAction>();
    // eos::print_str(" Read Action: ");
    // eos::print_name(act.deref().name);

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
