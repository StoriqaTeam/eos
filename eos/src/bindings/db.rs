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
    ///Update an association for a 64-bit integer secondary key to a primary key in a secondary 64-bit integer index table
    fn db_idx64_update(iterator: i32, payer: u64, secondary: *const u64);
    ///Remove a table row from a secondary 64-bit integer index table
    fn db_idx64_remove(iterator: i32);  
    ///Find the table row following the referenced table row in a secondary 64-bit integer index table 
    fn db_idx64_next( iterator: i32, primary: *const u64)  -> i32;
    ///Find the table row preceding the referenced table row in a secondary 64-bit integer index table 
    fn db_idx64_previous( iterator: i32, primary: *const u64)  -> i32;
    ///Find a table row in a secondary 64-bit integer index table by primary key 
    fn db_idx64_find_primary(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: u64)  -> i32;
    ///Find a table row in a secondary 64-bit integer index table by secondary key 
    fn db_idx64_find_secondary(code: u64, scope: u64, table: u64, secondary: *const u64, primary: *mut u64)  -> i32;
    ///Find the table row in a secondary 64-bit integer index table that matches the lowerbound condition for a given secondary key
    ///The table row that matches the lowerbound condition is the first table row in the table with the lowest secondary key that is >= the given key 
    fn db_idx64_lowerbound(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: *mut u64)  -> i32;
    ///Find the table row in a secondary 64-bit integer index table that matches the upperbound condition for a given secondary key
    ///The table row that matches the upperbound condition is the first table row in the table with the lowest secondary key that is > the given key 
    fn db_idx64_upperbound(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: *mut u64)  -> i32;
    ///Get an end iterator representing just-past-the-end of the last table row of a secondary 64-bit integer index table 
    fn db_idx64_end(code: u64, scope: u64,  table: u64)  -> i32;

    // NOTE: Currently Rust don't have proper FFI for x128 type

    //Store an association of a 128-bit integer secondary key to a primary key in a secondary 128-bit integer index table 
    //fn db_idx128_store(scope: u64, table: u64,  payer: u64, id: u64, secondary: *const u128)  -> i32;
    //Update an association for a 128-bit integer secondary key to a primary key in a secondary 128-bit integer index table 
    //fn db_idx128_update( iterator: i32,  payer: u64, secondary: *const u128); 
    //Remove a table row from a secondary 128-bit integer index table 
    //fn db_idx128_remove( iterator: i32); 
    //Find the table row following the referenced table row in a secondary 128-bit integer index table 
    //fn db_idx128_next( iterator: i32, primary: *const u64)  -> i32;
    //Find the table row preceding the referenced table row in a secondary 128-bit integer index table 
    //fn db_idx128_previous( iterator: i32, primary: *const u64)  -> i32;
    //Find a table row in a secondary 128-bit integer index table by primary key 
    //fn db_idx128_find_primary(code: u64, scope: u64, table: u64, secondary: *mut u128, primary: u64)  -> i32;
    //Find a table row in a secondary 128-bit integer index table by secondary key 
    //fn db_idx128_find_secondary(code: u64, scope: u64, table: u64, secondary: *const u128, primary: *mut u64)  -> i32;
    //Find the table row in a secondary 128-bit integer index table that matches the lowerbound condition for a given secondary key
    //The table row that matches the lowerbound condition is the first table row in the table with the lowest secondary key that is >= the given key 
    //fn db_idx128_lowerbound(code: u64, scope: u64, table: u64, secondary: *mut u128, primary: *mut u64)  -> i32;
    //Find the table row in a secondary 128-bit integer index table that matches the upperbound condition for a given secondary key
    //The table row that matches the upperbound condition is the first table row in the table with the lowest secondary key that is > the given key 
    //fn db_idx128_upperbound(code: u64, scope: u64, table: u64, secondary: *mut u128, primary: *mut u64)  -> i32;
    //Get an end iterator representing just-past-the-end of the last table row of a secondary 128-bit integer index table 
    //fn db_idx128_end(code: u64, scope: u64,  table: u64)  -> i32;

    // NOTE: Currently Rust don't have proper FFI for x128 type

    //Store an association of a 256-bit secondary key to a primary key in a secondary 256-bit index table 
    //fn db_idx256_store(scope: u64, table: u64,  payer: u64, id: u64, data: *const u128, data_len: usize )  -> i32;
    //Update an association for a 256-bit secondary key to a primary key in a secondary 256-bit index table 
    //fn db_idx256_update( iterator: i32,  payer: u64, data: *const u128, data_len: usize); 
    //Remove a table row from a secondary 256-bit index table 
    //fn db_idx256_remove( iterator: i32); 
    //Find the table row following the referenced table row in a secondary 256-bit index table 
    //fn db_idx256_next( iterator: i32, primary: *const u64)  -> i32;
    //Find the table row preceding the referenced table row in a secondary 256-bit index table 
    //fn db_idx256_previous( iterator: i32, primary: *const u64)  -> i32;
    //Find a table row in a secondary 256-bit index table by primary key 
    //fn db_idx256_find_primary(code: u64, scope: u64, table: u64, data: *mut u128, data_len: usize, primary: u64)  -> i32;
    //Find a table row in a secondary 256-bit index table by secondary key 
    //fn db_idx256_find_secondary(code: u64, scope: u64, table: u64, data: *const u128, data_len: usize, primary: *const u64)  -> i32;
    //Find the table row in a secondary 256-bit index table that matches the lowerbound condition for a given secondary key
    //The table row that matches the lowerbound condition is the first table row in the table with the lowest secondary key that is >= the given key (uses lexicographical ordering on the 256-bit keys) 
    //fn db_idx256_lowerbound(code: u64, scope: u64, table: u64, data: *mut u128, data_len: usize, primary: *const u64)  -> i32;
    //Find the table row in a secondary 256-bit index table that matches the upperbound condition for a given secondary key
    //The table row that matches the upperbound condition is the first table row in the table with the lowest secondary key that is > the given key (uses lexicographical ordering on the 256-bit keys) 
    //fn db_idx256_upperbound(code: u64, scope: u64, table: u64, data: *mut u128, data_len: usize, primary: *const u64)  -> i32;
    //Get an end iterator representing just-past-the-end of the last table row of a secondary 256-bit index table 
    //fn db_idx256_end(code: u64, scope: u64,  table: u64)  -> i32;


    ///Store an association of a double-precision floating-point secondary key to a primary key in a secondary double-precision floating-point index table 
    fn db_idx_double_store(scope: u64, table: u64,  payer: u64, id: u64, secondary: *const f64)  -> i32;
    ///Update an association for a double-precision floating-point secondary key to a primary key in a secondary double-precision floating-point index table 
    fn db_idx_double_update( iterator: i32,  payer: u64, secondary: *const f64); 
    ///Remove a table row from a secondary double-precision floating-point index table 
    fn db_idx_double_remove( iterator: i32); 
    ///Find the table row following the referenced table row in a secondary double-precision floating-point index table 
    fn db_idx_double_next( iterator: i32, primary: *const u64)  -> i32;
    ///Find the table row preceding the referenced table row in a secondary double-precision floating-point index table 
    fn db_idx_double_previous( iterator: i32, primary: *const u64)  -> i32;
    ///Find a table row in a secondary double-precision floating-point index table by primary key 
    fn db_idx_double_find_primary(code: u64, scope: u64, table: u64,secondary: *mut f64, primary: u64)  -> i32;
    ///Find a table row in a secondary double-precision floating-point index table by secondary key 
    fn db_idx_double_find_secondary(code: u64, scope: u64, table: u64, secondary: *const f64, primary: *mut u64)  -> i32;
    ///Find the table row in a secondary double-precision floating-point index table that matches the lowerbound condition for a given secondary key
    ///The table row that matches the lowerbound condition is the first table row in the table with the lowest secondary key that is >= the given key 
    fn db_idx_double_lowerbound(code: u64, scope: u64, table: u64,secondary: *mut f64, primary: *mut u64)  -> i32;
    ///Find the table row in a secondary double-precision floating-point index table that matches the upperbound condition for a given secondary key
    ///The table row that matches the upperbound condition is the first table row in the table with the lowest secondary key that is > the given key 
    fn db_idx_double_upperbound(code: u64, scope: u64, table: u64,secondary: *mut f64, primary: *mut u64)  -> i32;
    ///Get an end iterator representing just-past-the-end of the last table row of a secondary double-precision floating-point index table 
    fn db_idx_double_end(code: u64, scope: u64,  table: u64)  -> i32;
    
    // NOTE: Currently Rust don't have proper FFI for x128 type

    //Store an association of a quadruple-precision floating-point secondary key to a primary key in a secondary quadruple-precision floating-point index table 
    //fn db_idx_long_double_store(scope: u64, table: u64,  payer: u64, id: u64, secondary: *const f128)  -> i32;
    //Update an association for a quadruple-precision floating-point secondary key to a primary key in a secondary quadruple-precision floating-point index table 
    //fn db_idx_long_double_update( iterator: i32,  payer: u64, secondary: *const f128); 
    //Remove a table row from a secondary quadruple-precision floating-point index table 
    //fn db_idx_long_double_remove( iterator: i32); 
    //Find the table row following the referenced table row in a secondary quadruple-precision floating-point index table 
    //fn db_idx_long_double_next( iterator: i32, primary: *const u64)  -> i32;
    //Find the table row preceding the referenced table row in a secondary quadruple-precision floating-point index table 
    //fn db_idx_long_double_previous( iterator: i32, primary: *const u64)  -> i32;
    //Find a table row in a secondary quadruple-precision floating-point index table by primary key 
    //fn db_idx_long_double_find_primary(code: u64, scope: u64, table: u64, secondary: *mut f128, primary: u64)  -> i32;
    //Find a table row in a secondary quadruple-precision floating-point index table by secondary key 
    //fn db_idx_long_double_find_secondary(code: u64, scope: u64, table: u64, secondary: *const f128, primary: *const u64)  -> i32;
    //Find the table row in a secondary quadruple-precision floating-point index table that matches the lowerbound condition for a given secondary key
    //The table row that matches the lowerbound condition is the first table row in the table with the lowest secondary key that is >= the given key 
    //fn db_idx_long_double_lowerbound(code: u64, scope: u64, table: u64, secondary: *mut f128, primary: *const u64)  -> i32;
    //Find the table row in a secondary quadruple-precision floating-point index table that matches the upperbound condition for a given secondary key
    //The table row that matches the upperbound condition is the first table row in the table with the lowest secondary key that is > the given key 
    //fn db_idx_long_double_upperbound(code: u64, scope: u64, table: u64, secondary: *mut f128, primary: *const u64)  -> i32;
    //Get an end iterator representing just-past-the-end of the last table row of a secondary quadruple-precision floating-point index table 
    //fn db_idx_long_double_end(code: u64, scope: u64,  table: u64) -> i32;
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
pub fn db_association_i64_store(scope: AccountName, table: TableName, payer: AccountName, primary_key: u64, secondary_key: u64) -> TableRowIterator {
    unsafe {
        let iter = db_idx64_store(scope.0, table.0, payer.0, primary_key, secondary_key as *const u64);
        TableRowIterator(iter)
    }
}
///Update an association for a 64-bit integer secondary key to a primary key in a secondary 64-bit integer index table
pub fn db_association_i64_update(iter: TableRowIterator, payer: AccountName, secondary_key: u64) {
    unsafe {
        db_idx64_update(iter.0, payer.0, secondary_key as *const u64);
    }
}
///Remove a table row from a secondary 64-bit integer index table
pub fn db_association_i64_remove(iter: TableRowIterator) {
    unsafe {
        db_idx64_remove(iter.0);
    }
}
///Find the table row following the referenced table row in a secondary 64-bit integer index table 
pub fn db_association_i64_next(iter: TableRowIterator, primary_key: u64) -> TableRowIterator {
    unsafe {
        let iter = db_idx64_next(iter.0, primary_key as *const u64);
        TableRowIterator(iter)
    }
}
///Find the table row preceding the referenced table row in a secondary 64-bit integer index table 
pub fn db_association_i64_previous(iter: TableRowIterator, primary_key: u64) -> TableRowIterator {
    unsafe {
        let iter = db_idx64_previous(iter.0, primary_key as *const u64);
        TableRowIterator(iter)
    }
}
///Find a table row in a secondary 64-bit integer index table by primary key 
pub fn db_association_i64_find_primary(table_owner: AccountName, scope: AccountName, table: TableName, primary: u64) -> (u64, TableRowIterator) {
    unsafe {
        let secondary = 0;
        let iter = db_idx64_find_primary(table_owner.0, scope.0, table.0, secondary as *mut u64, primary);
        (secondary, TableRowIterator(iter))
    }
}
///Find a table row in a secondary 64-bit integer index table by secondary key 
pub fn db_association_i64_find_secondary(table_owner: AccountName, scope: AccountName, table: TableName, secondary: u64) -> (u64, TableRowIterator) {
    unsafe {
        let primary = 0;
        let iter = db_idx64_find_secondary(table_owner.0, scope.0, table.0, secondary as *const u64, primary as *mut u64);
        (primary, TableRowIterator(iter))
    }
}
///Find the table row in a secondary 64-bit integer index table that matches the lowerbound condition for a given secondary key
pub fn db_association_i64_lowerbound(table_owner: AccountName, scope: AccountName, table: TableName) -> (u64,  u64, TableRowIterator) {
    unsafe {
        let primary = 0;
        let secondary = 0;
        let iter = db_idx64_lowerbound(table_owner.0, scope.0, table.0, secondary as *mut u64, primary as *mut u64);
        (primary, secondary, TableRowIterator(iter))
    }
}
///Find the table row in a secondary 64-bit integer index table that matches the upperbound condition for a given secondary key
pub fn db_association_i64_upperbound(table_owner: AccountName, scope: AccountName, table: TableName) -> (u64, u64, TableRowIterator) {
    unsafe {
        let primary = 0;
        let secondary = 0;
        let iter = db_idx64_upperbound(table_owner.0, scope.0, table.0, secondary as *mut u64, primary as *mut u64);
        (primary, secondary, TableRowIterator(iter))
    }
}
///Get an end iterator representing just-past-the-end of the last table row of a secondary 64-bit integer index table 
pub fn db_association_i64_end(table_owner: AccountName, scope: AccountName, table: TableName) -> TableRowIterator {
    unsafe {
        let iter = db_idx64_end(table_owner.0, scope.0, table.0);
        TableRowIterator(iter)
    }
}

///Store an association of a double-precision floating-point secondary key to a primary key in a secondary double-precision floating-point index table 
pub fn db_association_f64_store(scope: AccountName, table: TableName, payer: AccountName, primary_key: u64, secondary_key: f64) -> TableRowIterator {
    unsafe {
        let iter = db_idx_double_store(scope.0, table.0, payer.0, primary_key, secondary_key as *const f64);
        TableRowIterator(iter)
    }
}
///Update an association for a double-precision floating-point secondary key to a primary key in a secondary double-precision floating-point index table 
pub fn db_association_f64_update(iter: TableRowIterator, payer: AccountName, secondary_key: f64) {
    unsafe {
        db_idx_double_update(iter.0, payer.0, secondary_key as *const f64);
    }
}
///Remove a table row from a secondary double-precision floating-point index table 
pub fn db_association_f64_remove(iter: TableRowIterator) {
    unsafe {
        db_idx_double_remove(iter.0);
    }
}
///Find the table row following the referenced table row in a secondary double-precision floating-point index table 
pub fn db_association_f64_next(iter: TableRowIterator, primary_key: u64) -> TableRowIterator {
    unsafe {
        let iter = db_idx_double_next(iter.0, primary_key as *const u64);
        TableRowIterator(iter)
    }
}
///Find the table row preceding the referenced table row in a secondary double-precision floating-point index table 
pub fn db_association_f64_previous(iter: TableRowIterator, primary_key: u64) -> TableRowIterator {
    unsafe {
        let iter = db_idx_double_previous(iter.0, primary_key as *const u64);
        TableRowIterator(iter)
    }
}
///Find a table row in a secondary double-precision floating-point index table by primary key 
pub fn db_association_f64_find_primary(table_owner: AccountName, scope: AccountName, table: TableName, primary: u64) -> (f64, TableRowIterator) {
    unsafe {
        let secondary = 0f64;
        let iter = db_idx_double_find_primary(table_owner.0, scope.0, table.0, secondary as *mut f64, primary);
        (secondary, TableRowIterator(iter))
    }
}
///Find a table row in a secondary double-precision floating-point index table by secondary key 
pub fn db_association_f64_find_secondary(table_owner: AccountName, scope: AccountName, table: TableName, secondary: f64) -> (u64, TableRowIterator) {
    unsafe {
        let primary = 0;
        let iter = db_idx_double_find_secondary(table_owner.0, scope.0, table.0, secondary as *const f64, primary as *mut u64);
        (primary, TableRowIterator(iter))
    }
}
///Find the table row in a secondary double-precision floating-point index table that matches the lowerbound condition for a given secondary key
pub fn db_association_f64_lowerbound(table_owner: AccountName, scope: AccountName, table: TableName) -> (u64,  f64, TableRowIterator) {
    unsafe {
        let primary = 0;
        let secondary = 0f64;
        let iter = db_idx_double_lowerbound(table_owner.0, scope.0, table.0, secondary as *mut f64, primary as *mut u64);
        (primary, secondary, TableRowIterator(iter))
    }
}
///Find the table row in a secondary double-precision floating-point index table that matches the upperbound condition for a given secondary key
pub fn db_association_f64_upperbound(table_owner: AccountName, scope: AccountName, table: TableName) -> (u64, f64, TableRowIterator) {
    unsafe {
        let primary = 0;
        let secondary = 0f64;
        let iter = db_idx_double_upperbound(table_owner.0, scope.0, table.0, secondary as *mut f64, primary as *mut u64);
        (primary, secondary, TableRowIterator(iter))
    }
}
///Get an end iterator representing just-past-the-end of the last table row of a secondary double-precision floating-point index table 
pub fn db_association_f64_end(table_owner: AccountName, scope: AccountName, table: TableName) -> TableRowIterator {
    unsafe {
        let iter = db_idx_double_end(table_owner.0, scope.0, table.0);
        TableRowIterator(iter)
    }
}
