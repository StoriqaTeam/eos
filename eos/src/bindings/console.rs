//! Defnes API to log/print text messages.
use alloc::string::String;

use error::Error;

extern "C" {
    // Prints string
    // fn prints(cstr: *const u8);
    // Prints string up to given length
    fn prints_l(cstr: *const u8, len: usize);

    // Prints value as a 64 bit signed integer
    fn printi(value: i64);
    // Prints value as a 64 bit unsigned integer
    fn printui(value: u64);

    // Prints value as a 128 bit signed integer
    //fn printi128(value: i128);            // NOTE: Currently Rust don't have proper FFI for x128 type
    // Prints value as a 128 bit unsigned integer
    //fn printui128(value: u128);           // NOTE: Currently Rust don't have proper FFI for x128 type

    // Prints value as single-precision floating point number
    fn printsf(value: f32);
    // Prints value as double-precision floating point number
    fn printdf(value: f64);

    // Prints value as quadruple-precision floating point number
    //fn printqf(value: f128);               // NOTE: Currently Rust don't have proper FFI for x128 type

    // Prints a 64 bit names as base32 encoded string
    fn printn(name: u64);

    // Prints hex representation of the data
    fn printhex(data: *const u8, data_len: usize);
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

/// Print i128
// pub fn print_i128(i: i128) {
//     unsafe {
//         printi128(i);
//     }
// }

/// Print u128
// pub fn print_u128(u: u128) {
//     unsafe {
//         printui128(u);
//     }
// }

/// Print f32
pub fn print_f32(f: f32) {
    unsafe {
        printsf(f);
    }
}

/// Print f64
pub fn print_f64(df: f64) {
    unsafe {
        printdf(df);
    }
}

/// Print f128
// pub fn print_f128(qf: f128) {
//     unsafe {
//         printqf(qf);
//     }
// }

/// Print string
pub fn print_str(s: &str) {
    unsafe {
        let bytes = s.as_bytes();
        prints_l(bytes.as_ptr(), bytes.len());
    }
}

/// Print string with given length
pub fn print_str_with_len(s: &str, len: usize) {
    unsafe {
        let bytes = s.as_bytes();
        prints_l(bytes.as_ptr(), len);
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

#[inline]
fn base32_to_byte(b: u8) -> u8 {
    match b {
        6...31 => b + 97 - 6,
        1...6 => b + 49 - 1,
        _ => 0,
    }
}

/// Convert base_32 to str
pub fn name_to_str(name: u64) -> Result<String, Error> {
    let mut slice = [0; 12];
    for i in 0..12 {
        let mut mask: u64 = 0b0001_1111 << 5 * i;
        let mut b = name & mask;
        b >>= 5 * i;
        let s = base32_to_byte(b as u8);
        slice[i] = s;
    }
    match String::from_utf8(slice.to_vec()) {
        Ok(s) => Ok(s),
        Err(_) => Err(Error::Utf8Error),
    }
}

/// Print string with given length
pub fn print_hex(s: &str, len: usize) {
    unsafe {
        let bytes = s.as_bytes();
        printhex(bytes.as_ptr(), len);
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
