//! Defines API for querying internal chain state
use core::alloc::{GlobalAlloc, Layout};

use types::*;
use GLOBAL_ALLOCATOR;

extern "C" {
    /// Tests if the sha256 hash generated from data matches the provided checksum.
    fn assert_sha256(data: *const Opaque, length: usize, hash: *const Opaque);
    /// Tests if the sha1 hash generated from data matches the provided checksum.
    fn assert_sha1(data: *const Opaque, length: usize, hash: *const Opaque);
    /// Tests if the sha512 hash generated from data matches the provided checksum.
    fn assert_sha512(data: *const Opaque, length: usize, hash: *const Opaque);
    /// Tests if the ripemod160 hash generated from data matches the provided checksum.
    fn assert_ripemd160(data: *const Opaque, length: usize, hash: *const Opaque);
    /// Hashes data using sha256 and stores result in memory pointed to by hash.
    fn sha256(data: *const Opaque, length: usize, hash: *mut Opaque);
    /// Hashes data using sha1 and stores result in memory pointed to by hash.
    fn sha1(data: *const Opaque, length: usize, hash: *mut Opaque);
    /// Hashes data using sha512 and stores result in memory pointed to by hash.
    fn sha512(data: *const Opaque, length: usize, hash: *mut Opaque);
    /// Hashes data using ripemod160 and stores result in memory pointed to by hash.
    fn ripemd160(data: *const Opaque, length: usize, hash: *mut Opaque);
    /// Calculates the public key used for a given signature and hash used to create a message.
    fn recover_key(digest: *const Opaque, sig: *const Opaque, siglen: usize, public_key: *mut Opaque, publen: usize) -> i32;
    /// Tests a given public key with the generated key from digest and the signature.
    fn assert_recover_key(digest: *const Opaque, sig: *const Opaque, siglen: usize, public_key: *const Opaque, publen: usize);
}

/// Tests if the sha256 hash generated from data matches the provided checksum.
pub fn assert_eq_sha256(data: &[Opaque], hash: CheckSum256) {
    unsafe { assert_sha256(data.as_ptr(), data.len(), hash.as_ptr()) }
}
/// Tests if the sha1 hash generated from data matches the provided checksum.
pub fn assert_eq_sha1(data: &[Opaque], hash: CheckSum160) {
    unsafe { assert_sha1(data.as_ptr(), data.len(), hash.as_ptr()) }
}
/// Tests if the sha512 hash generated from data matches the provided checksum.
pub fn assert_eq_sha512(data: &[Opaque], hash: CheckSum512) {
    unsafe { assert_sha512(data.as_ptr(), data.len(), hash.as_ptr()) }
}
/// Tests if the ripemod160 hash generated from data matches the provided checksum.
pub fn assert_eq_ripemd160(data: &[Opaque], hash: CheckSum160) {
    unsafe { assert_ripemd160(data.as_ptr(), data.len(), hash.as_ptr()) }
}
/// Hashes data using sha256 and stores result in memory pointed to by hash.
pub fn generate_sha256(data: &[Opaque]) -> CheckSum256 {
    unsafe {
        let hash = [0; 32];
        sha256(data.as_ptr(), data.len(), hash.as_ptr() as *mut Opaque);
        CheckSum256::new(hash)
    }
}
/// Hashes data using sha1 and stores result in memory pointed to by hash.
pub fn generate_sha1(data: &[Opaque]) -> CheckSum160 {
    unsafe {
        let hash = [0; 20];
        sha1(data.as_ptr(), data.len(), hash.as_ptr() as *mut Opaque);
        CheckSum160::new(hash)
    }
}
/// Hashes data using sha512 and stores result in memory pointed to by hash.
pub fn generate_sha512(data: &[Opaque]) -> CheckSum512 {
    unsafe {
        let hash = [0; 64];
        sha512(data.as_ptr(), data.len(), hash.as_ptr() as *mut Opaque);
        CheckSum512::new(hash)
    }
}
/// Hashes data using ripemod160 and stores result in memory pointed to by hash.
pub fn generate_ripemd160(data: &[Opaque]) -> CheckSum160 {
    unsafe {
        let hash = [0; 20];
        ripemd160(data.as_ptr(), data.len(), hash.as_ptr() as *mut Opaque);
        CheckSum160::new(hash)
    }
}
/// Calculates the public key used for a given signature and hash used to create a message.
pub fn calc_recover_key<'a>(digest: &'a [Opaque], sig: &'a [Opaque], public_key_length: usize) -> &'a [Opaque] {
    unsafe {
        let align = 1; // 1 byte
        let layout = Layout::from_size_align(public_key_length, align).unwrap();
        let ptr = GLOBAL_ALLOCATOR.alloc(layout);
        recover_key(digest.as_ptr(), sig.as_ptr(), sig.len(), ptr, public_key_length);
        ::core::slice::from_raw_parts(ptr, public_key_length)
    }
}
/// Tests a given public key with the generated key from digest and the signature.
pub fn test_recover_key(digest: &[Opaque], sig: &[Opaque], public_key: &[Opaque]) {
    unsafe { assert_recover_key(digest.as_ptr(), sig.as_ptr(), sig.len(), public_key.as_ptr(), public_key.len()) }
}
