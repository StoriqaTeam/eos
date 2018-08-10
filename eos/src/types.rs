//! Common Rust types used for shadowing C types from eos
use core::fmt;

/// Opaque byte
pub type Opaque = u8;

/// typedef uint64_t account_name
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, new)]
pub struct AccountName(u64);
/// typedef uint64_t table_name
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, new)]
pub struct TableName(u64);
/// typedef uint64_t permission_name
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, new)]
pub struct PermissionName(u64);
/// typedef uint64_t action_name;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, new)]
pub struct ActionName(u64);

/// uint8_t hash[32];
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, new)]
pub struct CheckSum256([u8; 32]);

/// uint8_t hash[20];
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, new)]
pub struct CheckSum160([u8; 20]);

/// uint8_t hash[64];
#[repr(C)]
#[derive(Clone, Copy, Deref, new)]
pub struct CheckSum512([u8; 64]);

/// Strange, but Debug is not implemented for [u8; 64] by default
impl fmt::Debug for CheckSum512 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&&self.0[..], f)
    }
}

/// Iterator points to an existing table row in the table
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, new)]
pub struct TableRowIterator(i32);

/// Primary table key
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, new)]
pub struct PrimaryKey(u64);

/// Secondary table key u64
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, new)]
pub struct SecondaryKeyU64(u64);

/// Secondary table key f64
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Deref, new)]
pub struct SecondaryKeyF64(f64);
