extern "C" {
    fn printi(c: i64);
    fn printui(c: u64);
    fn prints_l(bytes: *const u8, len: i32);
    fn printn(name: u64);
    fn read_action_data(bytes: *mut u8, len: u32) -> u32;
}

use core::cell::Cell;

pub fn read_action_bytes() -> &[u8] {}

pub fn print_i64(i: i64) {
    unsafe {
        printi(i);
    }
}

pub fn print_u64(u: u64) {
    unsafe {
        printui(u);
    }
}

pub fn print_str(s: &str) {
    unsafe {
        let bytes = s.as_bytes();
        prints_l(bytes.as_ptr(), bytes.len() as i32);
    }
}

pub fn print_name(name: u64) {
    unsafe {
        printn(name);
    }
}

pub fn str_to_name(s: &str) -> u64 {
    let mut res: u64 = 0;
    let mut bytes = s.bytes();
    for i in 0..12 {
        let mut b: u64 = 0;
        if let Some(c) = bytes.next() {
            b = byte_to_base32(c) as u64;
        }
        if i < 12 {
            b &= 0x1f;
            b <<= 64 - 5 * (i + 1);
        } else {
            b &= 0x0f;
        }
        res |= b;
    }
    res
}

#[inline]
fn byte_to_base32(b: u8) -> u8 {
    match b {
        97...122 => b - 97 + 6,
        49...54 => b - 49 + 1,
        _ => 0,
    }
}
