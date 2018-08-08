//! Defnes API to log/print text messages.
use alloc::prelude::ToString;
use alloc::string::String;

use types::Opaque;

extern "C" {
    fn printi(c: i64);
    fn printui(c: u64);
    fn prints_l(bytes: *const u8, len: i32);
    fn printn(name: u64);
}

/// Print i64
pub fn print_i64(i: i64) {
    unsafe {
        printi(i);
    }
}

/// Print u64
pub fn print_u64(u: u64) {
    unsafe {
        printui(u);
    }
}

/// Print string
pub fn print_str(s: &str) {
    unsafe {
        let bytes = s.as_bytes();
        prints_l(bytes.as_ptr(), bytes.len() as i32);
    }
}

/// Print name
pub fn print_name(name: u64) {
    unsafe {
        printn(name);
    }
}

/// Convert str to u64
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

/// Convert str to u64
pub fn name_to_str(name: u64) -> String {
    unsafe {
        let raw_ptr = name as *const Opaque;
        let size: usize = 8;
        let slice = ::core::slice::from_raw_parts(raw_ptr, size);
        String::from_utf8_lossy(&slice).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_to_base32_test() {
        assert_eq!(byte_to_base32(0), 0);
        assert_eq!(byte_to_base32(49), 1);
        assert_eq!(byte_to_base32(54), 5);
        assert_eq!(byte_to_base32(97), 6);
        assert_eq!(byte_to_base32(122), 32);
        assert_eq!(byte_to_base32(123), 0);
    }
}
