mod deserialize;

pub use self::deserialize::{Deserialize, Reader};
use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use error::Error;
use ALLOC;
use core::mem::transmute;

type Opaque = u8;

extern "C" {
    fn printi(c: i64);
    fn printui(c: u64);
    fn prints_l(bytes: *const u8, len: i32);
    fn printn(name: u64);

    fn action_data_size() -> u32;
    fn read_action_data(bytes: *mut Opaque, len: u32) -> u32;

    fn db_store_i64(
        scope: u64,
        table: u64,
        payer: u64,
        id: u64,
        data: *const Opaque,
        len: u32,
    ) -> i32;
    fn db_get_i64(iterator: i32, data: *mut Opaque, len: u32) -> i32;
    fn db_find_i64(code: u64, scope: u64, table: u64, id: u64) -> i32;
}

pub fn store_bytes(scope: u64, table: u64, payer: u64, id: u64, data: &[u8]) {
    // let ptr = data.as_ptr();
    let len = data.len();
    let bytes: [u8; 8] = unsafe { transmute(id.to_be()) };
    let mut d: Vec<u8> = Vec::with_capacity(8 + len);
    d.extend(bytes.into_iter());
    d.extend(data);
    print_str(" ");
    print_u64(d[0] as u64);
    print_str(" ");
    print_u64(d[1] as u64);
    print_str(" ");
    print_u64(d[2] as u64);
    print_str(" ");
    print_u64(d[3] as u64);
    print_str(" ");
    print_u64(d[4] as u64);
    print_str(" ");
    print_u64(d[5] as u64);
    print_str(" ");
    print_u64(d[6] as u64);
    print_str(" ");
    print_u64(d[7] as u64);
    print_str(" ");
    print_u64(d[8] as u64);
    print_str(" ");
    print_u64(d[9] as u64);
    print_str(" ");
    let ptr = d.as_ptr();
    let iter = unsafe {
        db_store_i64(scope, table, payer, id, ptr, len as u32)
    };
    print_str("Iter: ");
    print_i64(iter as i64);
}

pub fn read_bytes(table_owner: u64, scope: u64, table: u64, id: u64) -> Vec<u8> {
    unsafe {
        let len = 256;
        print_str(" Params: ");
        print_str(" table_owner: ");
        print_name(table_owner);
        print_str(" scope: ");
        print_name(scope);
        print_str(" table: ");
        print_name(table);
        print_str(" id: ");
        print_name(id);
        let iter = db_find_i64(table_owner, scope, table, id);
        let mut res: Vec<u8> = Vec::with_capacity(len);
        db_get_i64(iter, res.as_mut_slice().as_mut_ptr(), len as u32);
        res
    }
}

pub fn read_action<T: Deserialize>() -> Result<T, Error> {
    unsafe {
        let size = action_data_size() as usize;
        let align = 1; // 1 byte
        let layout = Layout::from_size_align(size, align).unwrap();
        let ptr = ALLOC.alloc(layout);
        read_action_data(ptr, size as u32);
        let slice = ::core::slice::from_raw_parts(ptr, size);
        let mut deserializer = Reader::new(slice);
        <T as Deserialize>::deserialize(deserializer)
    }
}

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
