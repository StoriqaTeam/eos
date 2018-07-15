mod deserialize;

pub use self::deserialize::{Deserialize, Reader};
use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use error::Error;
use ALLOC;

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
    fn db_update_i64(iterator: i32, payer: u64, data: *mut Opaque, len: u32);
    fn db_find_i64(code: u64, scope: u64, table: u64, id: u64) -> i32;
}

pub fn db_store<T>(scope: u64, table: u64, payer: u64, id: u64, data: &T) {
    unsafe {
        db_store_i64(scope, table, payer, id, (data as *const T) as *const Opaque, ::core::mem::size_of::<T>() as u32)
    };
}

pub fn db_store_bytes(scope: u64, table: u64, payer: u64, id: u64, data: &[u8]) {
    let ptr = data.as_ptr();
    let len = data.len();
    unsafe {
        db_store_i64(scope, table, payer, id, ptr, len as u32)
    };
}

pub fn db_update<T>(table_owner: u64, scope: u64, payer: u64, table: u64, id: u64, data: &mut T) {
    unsafe {
        let iter = db_find_i64(table_owner, scope, table, id);
        db_update_i64(iter, payer, (data as *mut T) as *mut Opaque, ::core::mem::size_of::<T>() as u32);
    };
}


pub fn db_read<T: Deserialize>(table_owner: u64, scope: u64, table: u64, id: u64) -> Result<T, Error> {
    unsafe {
        let iter = db_find_i64(table_owner, scope, table, id);
        let size = ::core::mem::size_of::<T>();
        let align = 1; // 1 byte
        let layout = Layout::from_size_align(size, align).unwrap();
        let ptr = ALLOC.alloc(layout);
        let sz = db_get_i64(iter, ptr, size as u32);
        let slice = ::core::slice::from_raw_parts(ptr, size);
        let mut deserializer = Reader::new(slice);
        <T as Deserialize>::deserialize(deserializer)
    }
}


pub fn db_read_bytes(table_owner: u64, scope: u64, table: u64, id: u64) -> Vec<u8> {
    unsafe {
        let iter = db_find_i64(table_owner, scope, table, id);
        let size = db_get_i64(iter, null_mut(), 0);
        let mut res: Vec<u8> = Vec::with_capacity(size as usize);
        res.set_len(size as usize);
        db_get_i64(iter, res.as_mut_slice().as_mut_ptr(), size as u32);
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
