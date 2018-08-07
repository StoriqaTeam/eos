//! Common Rust types used for shadowing C types from eos

/// Opaque byte
pub type Opaque = u8;
/// typedef uint64_t account_name
pub type AccountName = u64;
/// typedef uint64_t action_name;
pub type ActionName = u64;
/// typedef uint64_t permission_name
pub type PermissionName = u64;
/// typedef uint64_t scope_name;
pub type ScopeName = u64;
