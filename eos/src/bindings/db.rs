//! Defines API for interfacing with blockchain database

use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use deserialize::{Deserialize, Reader};
use error::Error;
use types::*;
use GLOBAL_ALLOCATOR;

extern "C" {
    ///Store a record in a primary 64-bit integer index table
    fn db_store_i64(scope: u64, table: u64, payer: u64, id: u64, data: *const Opaque, len: u32) -> i32;
    ///Get a record in a primary 64-bit integer index table
    fn db_get_i64(iterator: i32, data: *mut Opaque, len: u32) -> i32;
    ///Update a record in a primary 64-bit integer index table
    fn db_update_i64(iterator: i32, payer: u64, data: *mut Opaque, len: u32);
    ///Remove a record from a primary 64-bit integer index table
    fn db_remove_i64(iterator: i32);
    ///Find the table row following the referenced table row in a primary 64-bit integer index table
    fn db_next_i64(iterator: i32, primary: *mut u64) -> i32;
    ///Find the table row preceding the referenced table row in a primary 64-bit integer index table
    fn db_previous_i64(iterator: i32, primary: *mut u64) -> i32;
    ///Find a table row in a primary 64-bit integer index table by primary key
    fn db_find_i64(code: u64, scope: u64, table: u64, id: u64) -> i32;
    ///Find the table row in a primary 64-bit integer index table that matches the lowerbound condition for a given primary key
    fn db_lowerbound_i64(code: u64, scope: u64, table: u64, id: u64) -> i32;
    ///Find the table row in a primary 64-bit integer index table that matches the upperbound condition for a given primary key
    fn db_upperbound_i64(code: u64, scope: u64, table: u64, id: u64) -> i32;
    ///Get an iterator representing just-past-the-end of the last table row of a primary 64-bit integer index table
    fn db_end_i64(code: u64, scope: u64, table: u64) -> i32;
    ///Store an association of a 64-bit integer secondary key to a primary key in a secondary 64-bit integer index table
    fn db_idx64_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const u64) -> i32;
   
}

/// Store object of type T in db
pub fn db_store<T>(scope: AccountName, table: TableName, payer: AccountName, id: u64, data: &T) -> TableRowIterator {
    unsafe {
        let raw_data: *const T = data;
        let iter = db_store_i64(
            scope.0,
            table.0,
            payer.0,
            id,
            raw_data as *const Opaque,
            ::core::mem::size_of::<T>() as u32,
        );
        TableRowIterator(iter)
    }
}

/// Store bytes in db
pub fn db_store_bytes(scope: AccountName, table: TableName, payer: AccountName, id: u64, data: &[u8]) -> TableRowIterator {
    let ptr = data.as_ptr();
    let len = data.len();
    unsafe {
        let iter = db_store_i64(scope.0, table.0, payer.0, id, ptr, len as u32);
        TableRowIterator(iter)
    }
}

/// Update stored object in db
pub fn db_update<T>(table_owner: AccountName, scope: AccountName, payer: AccountName, table: TableName, id: u64, data: &mut T) {
    unsafe {
        let iter = db_find_i64(table_owner.0, scope.0, table.0, id);
        let raw_data: *mut T = data;
        db_update_i64(iter, payer.0, raw_data as *mut Opaque, ::core::mem::size_of::<T>() as u32);
    };
}

/// Remove a record from a primary 64-bit integer index table
pub fn db_remove(table_owner: AccountName, scope: AccountName, table: TableName, id: u64) {
    unsafe {
        let iter = db_find_i64(table_owner.0, scope.0, table.0, id);
        db_remove_i64(iter);
    };
}

///Find the table row following the referenced table row in a primary 64-bit integer index table
pub fn db_next_row(table_owner: AccountName, scope: AccountName, table: TableName, id: u64) -> TableRowIterator {
    unsafe {
        let iter = db_find_i64(table_owner.0, scope.0, table.0, id);
        let primary = 0;
        let iter = db_next_i64(iter, primary as *mut u64);
        TableRowIterator(iter)
    }
}

///Find the table row preceding the referenced table row in a primary 64-bit integer index table
pub fn db_previous_row(table_owner: AccountName, scope: AccountName, table: TableName, id: u64) -> TableRowIterator {
    unsafe {
        let iter = db_find_i64(table_owner.0, scope.0, table.0, id);
        let primary = 0;
        let iter = db_previous_i64(iter, primary as *mut u64);
        TableRowIterator(iter)
    }
}

/// After we polish the basics with allocation and serialization,
/// we need to figure out how to work with db indexes in EOS.
pub fn db_read<T: Deserialize>(table_owner: AccountName, scope: AccountName, table: TableName, id: u64) -> Result<T, Error> {
    unsafe {
        let iter = db_find_i64(table_owner.0, scope.0, table.0, id);
        let size = ::core::mem::size_of::<T>();
        let align = 1; // 1 byte
        let layout = Layout::from_size_align(size, align).unwrap();
        let ptr = GLOBAL_ALLOCATOR.alloc(layout);
        let _sz = db_get_i64(iter, ptr, size as u32);
        let slice = ::core::slice::from_raw_parts(ptr, size);
        let deserializer = Reader::new(slice);
        <T as Deserialize>::deserialize(deserializer)
    }
}

/// Read raw bytes from db
pub fn db_read_bytes(table_owner: AccountName, scope: AccountName, table: TableName, id: u64) -> Vec<u8> {
    unsafe {
        let iter = db_find_i64(table_owner.0, scope.0, table.0, id);
        let size = db_get_i64(iter, null_mut(), 0);
        let mut res: Vec<u8> = Vec::with_capacity(size as usize);
        res.set_len(size as usize);
        db_get_i64(iter, res.as_mut_slice().as_mut_ptr(), size as u32);
        res
    }
}

///Find a table row in a primary 64-bit integer index table by primary key
pub fn db_find(table_owner: AccountName, scope: AccountName, table: TableName, id: u64) -> TableRowIterator {
    unsafe {
        let iter = db_find_i64(table_owner.0, scope.0, table.0, id);
        TableRowIterator(iter)
    }
}

///Find the table row in a primary 64-bit integer index table that matches the lowerbound condition for a given primary key
pub fn db_lowerbound(table_owner: AccountName, scope: AccountName, table: TableName, id: u64) -> TableRowIterator {
    unsafe {
        let iter = db_lowerbound_i64(table_owner.0, scope.0, table.0, id);
        TableRowIterator(iter)
    }
}

///Find the table row in a primary 64-bit integer index table that matches the upperbound condition for a given primary key
pub fn db_upperbound(table_owner: AccountName, scope: AccountName, table: TableName, id: u64) -> TableRowIterator {
    unsafe {
        let iter = db_upperbound_i64(table_owner.0, scope.0, table.0, id);
        TableRowIterator(iter)
    }
}

///Get an iterator representing just-past-the-end of the last table row of a primary 64-bit integer index table
pub fn db_end(table_owner: AccountName, scope: AccountName, table: TableName) -> TableRowIterator {
    unsafe {
        let iter = db_end_i64(table_owner.0, scope.0, table.0);
        TableRowIterator(iter)
    }
}

///Store an association of a 64-bit integer secondary key to a primary key in a secondary 64-bit integer index table
pub fn db_store_association(scope: AccountName, table: TableName, payer: AccountName, primary_key: u64, secondary_key: u64) -> TableRowIterator {
    unsafe {
        let iter = db_idx64_store(scope.0, table.0, payer.0, primary_key, secondary_key as *const u64);
        TableRowIterator(iter)
    }
}
