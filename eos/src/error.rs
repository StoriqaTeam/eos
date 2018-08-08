//! Error module.

/// Custom library error.
#[derive(Debug, Copy, Clone)]
pub enum Error {
    /// Memory of smart contract is out of the bounds.
    MemoryOutOfBounds,
    /// Strings in UTF8 representation error.
    Utf8Error,
}
