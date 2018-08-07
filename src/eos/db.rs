//! Defines API for interfacing with blockchain database

use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use eos::deserialize::{Deserialize, Reader};
use eos::types::Opaque;
use error::Error;
use ALLOC;

extern "C" {
    fn db_store_i64(scope: u64, table: u64, payer: u64, id: u64, data: *const Opaque, len: u32) -> i32;
    fn db_get_i64(iterator: i32, data: *mut Opaque, len: u32) -> i32;
    fn db_update_i64(iterator: i32, payer: u64, data: *mut Opaque, len: u32);
    fn db_find_i64(code: u64, scope: u64, table: u64, id: u64) -> i32;
}

/// Store object of type T in db
pub fn db_store<T>(scope: u64, table: u64, payer: u64, id: u64, data: &T) {
    unsafe {
        let raw_data: *const T = data;
        db_store_i64(
            scope,
            table,
            payer,
            id,
            raw_data as *const Opaque,
            ::core::mem::size_of::<T>() as u32,
        )
    };
}

/// Store bytes in db
pub fn db_store_bytes(scope: u64, table: u64, payer: u64, id: u64, data: &[u8]) {
    let ptr = data.as_ptr();
    let len = data.len();
    unsafe { db_store_i64(scope, table, payer, id, ptr, len as u32) };
}

/// Update stored object in db
pub fn db_update<T>(table_owner: u64, scope: u64, payer: u64, table: u64, id: u64, data: &mut T) {
    unsafe {
        let iter = db_find_i64(table_owner, scope, table, id);
        let raw_data: *mut T = data;
        db_update_i64(iter, payer, raw_data as *mut Opaque, ::core::mem::size_of::<T>() as u32);
    };
}

/// After we polish the basics with allocation and serialization,
/// we need to figure out how to work with db indexes in EOS.
pub fn db_read<T: Deserialize>(table_owner: u64, scope: u64, table: u64, id: u64) -> Result<T, Error> {
    unsafe {
        let iter = db_find_i64(table_owner, scope, table, id);
        let size = ::core::mem::size_of::<T>();
        let align = 1; // 1 byte
        let layout = Layout::from_size_align(size, align).unwrap();
        let ptr = ALLOC.alloc(layout);
        let _sz = db_get_i64(iter, ptr, size as u32);
        let slice = ::core::slice::from_raw_parts(ptr, size);
        let deserializer = Reader::new(slice);
        <T as Deserialize>::deserialize(deserializer)
    }
}

/// Read raw bytes from db
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
