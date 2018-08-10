//! Defines API for querying action and sending action
use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use error::Error;

use deserialize::{Deserialize, Reader};
use types::*;
use GLOBAL_ALLOCATOR;

/// Defines API for querying action and sending action.
///  struct action {
///   scope_name scope; // the contract defining the primary code to execute for code/type
///   action_name name; // the action to be taken
///   permission_level[] authorization; // the accounts and permission levels provided
///   bytes data; // opaque data processed by code
/// };
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Action {
    scope: AccountName,                  // the contract defining the primary code to execute for code/type
    name: ActionName,                    // the action to be taken
    authorization: Vec<PermissionLevel>, // the accounts and permission levels provided
    data: *const Opaque,                 // opaque data processed by code
}

/// Permission level     
/// {
///   "name": "permission_level",
///   "base": "",
///   "fields": [
///     {"name": "actor", "type": "account_name"},
///     {"name": "permission", "type": "permission_name"}
///   ]
/// }
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PermissionLevel {
    name: ActionName,           // the action to be taken
    permission: PermissionName, // the permission to be taken
}

extern "C" {
    /// Copy current action data to the specified location.
    fn read_action_data(msg: *mut Opaque, len: usize) -> u32;
    /// Get the length of current action's data field.
    fn action_data_size() -> u32;
    /// Verifies that name has auth.
    fn has_auth(name: u64) -> bool;
    /// Verify specified account exists in the set of provided auths.
    fn require_auth(name: u64);
    /// Verify specified account exists in the set of provided auths.
    fn require_auth2(name: u64, permission: u64);
    /// Send an inline action in the context of this action's parent transaction.
    fn send_inline(serialized_action: *const Opaque, size: usize);
    /// Send an inline context free action in the context of this action's parent transaction.
    fn send_context_free_inline(serialized_action: *const Opaque, size: usize);
    /// Verifies that name exists in the set of write locks held.
    fn require_write_lock(name: u64);
    /// Verifies that name exists in the set of read locks held.
    fn require_read_lock(name: u64);
    /// Get the publication time.
    fn publication_time() -> u64;
    /// Get the current receiver of the action.
    fn current_receiver() -> u64;
}

/// Read action
pub fn read_action<T: Deserialize>() -> Result<T, Error> {
    unsafe {
        let size = action_data_size() as usize;
        let align = 1; // 1 byte
        let layout = Layout::from_size_align(size, align).unwrap();
        let ptr = GLOBAL_ALLOCATOR.alloc(layout);
        read_action_data(ptr, size);
        let slice = ::core::slice::from_raw_parts(ptr, size);
        let deserializer = Reader::new(slice);
        <T as Deserialize>::deserialize(deserializer)
    }
}

/// Read action
pub fn action_data_length() -> u32 {
    unsafe { action_data_size() }
}

/// Verifies that name has auth.
pub fn account_has_auth(name: AccountName) -> bool {
    unsafe { has_auth(*name) }
}

/// Verify specified account exists in the set of provided auths.
pub fn account_require_auth(name: AccountName) {
    unsafe { require_auth(*name) }
}
/// Verify specified account exists in the set of provided auths.
pub fn account_require_auth2(name: AccountName, permission: PermissionName) {
    unsafe { require_auth2(*name, *permission) }
}
/// Send an inline action in the context of this action's parent transaction.
pub fn action_send_inline(actions: &[Action]) {
    unsafe { send_inline(actions.as_ptr() as *const Opaque, actions.len()) }
}
/// Send an inline context free action in the context of this action's parent transaction.
pub fn action_send_context_free_inline(actions: &[Action]) {
    unsafe { send_context_free_inline(actions.as_ptr() as *const Opaque, actions.len()) }
}
/// Verifies that name exists in the set of write locks held.
pub fn verify_write_lock_exists(name: AccountName) {
    unsafe { require_write_lock(*name) }
}
/// Verifies that name exists in the set of read locks held.
pub fn verify_read_lock_exists(name: AccountName) {
    unsafe { require_read_lock(*name) }
}
/// Get the publication time.
pub fn publication_time_mcs() -> u64 {
    unsafe { publication_time() }
}
/// Get the current receiver of the action.
pub fn action_current_receiver() -> AccountName {
    unsafe { AccountName::new(current_receiver()) }
}
