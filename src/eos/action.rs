// use alloc::vec::Vec;
// use core::alloc::{GlobalAlloc, Layout};
// use core::ptr::null_mut;
// use error::Error;

// use ALLOC;
// use super::Opaque;

// extern "C" {
//     ///Copy current action data to the specified location.
//     fn  read_action_data (msg: *mut Opaque ,len: u32) -> u32;
//     /// Get the length of current action's data field.
//     fn  action_data_size() -> u32;
//     /// Add the specified account to set of accounts to be notified.
//     fn  require_recipient(account_namename);
//     /// Verify specified account exists in the set of provided auths.
//     fn  require_auth(account_namename);
//     /// Verifies that name has auth.
//     fn  has_auth(account_namename) -> bool;
//     /// Verify specified account exists in the set of provided auths.
//     fn  require_auth2(account_namename,permission_namepermission);
//     ///
//     fn  is_account(account_namename) -> bool;
//     /// Send an inline action in the context of this action's parent transaction.
//     fn  send_inline(char * serialized_action,size_t size);
//     /// Send an inline context free action in the context of this action's parent transaction.
//     fn  send_context_free_inline(char * serialized_action,size_t size);
//     /// Verifies that name exists in the set of write locks held.
//     fn  require_write_lock(account_namename);
//     /// Verifies that name exists in the set of read locks held.
//     fn  require_read_lock(account_namename);
//     /// Get the publication time.
//     fn  publication_time() ->  u64;
//     /// Get the current receiver of the action.
//     fn current_receiver() -> account_name
// }
