//! Common Rust types used for shadowing C types from eos

/// Opaque byte
pub type Opaque = u8;

/// typedef uint64_t account_name
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountName(pub u64);
/// typedef uint64_t permission_name
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PermissionName(pub u64);
/// typedef uint64_t action_name;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionName(pub u64);
