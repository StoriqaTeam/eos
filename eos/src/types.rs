//! Common Rust types used for shadowing C types from eos
use core::fmt;

/// Opaque byte
pub type Opaque = u8;

/// typedef uint64_t account_name
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountName(pub u64);
/// typedef uint64_t table_name
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableName(pub u64);
/// typedef uint64_t permission_name
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PermissionName(pub u64);
/// typedef uint64_t action_name;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionName(pub u64);

/// uint8_t hash[32];
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CheckSum256(pub [u8; 32]);

/// uint8_t hash[20];
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CheckSum160(pub [u8; 20]);

/// uint8_t hash[64];
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CheckSum512(pub [u8; 64]);

/// Strange, but Debug is not implemented for [u8; 64] by default
impl fmt::Debug for CheckSum512 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&&self.0[..], f)
    }
}

/// Iterator points to an existing table row in the table
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableRowIterator(pub i32);

/// Primary table key
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimaryKey(pub u64);

/// Secondary table key u64
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecondaryKeyU64(pub u64);

/// Secondary table key f64
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecondaryKeyF64(pub f64);
