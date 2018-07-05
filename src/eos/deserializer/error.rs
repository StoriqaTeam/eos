use alloc::boxed::Box;
use core::fmt::{self, Display};

/// The result of a serialization or deserialization operation.
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error that can be produced during (de)serializing.
pub type Error = Box<ErrorKind>;

/// The kind of error that can be produced during a serialization or deserialization.
#[derive(Debug)]
#[repr(C)]
pub enum ErrorKind {
    /// If the error stems from the reader/writer that is being used
    /// during (de)serialization, that error will be stored and returned here.
    Io,
    /// Returned if the deserializer attempts to deserialize a string that is not valid utf8
    InvalidUtf8Encoding,
    /// Returned if the deserializer attempts to deserialize a bool that was
    /// not encoded as either a 1 or a 0
    InvalidBoolEncoding,
    /// Returned if the deserializer attempts to deserialize a char that is not in the correct format.
    InvalidCharEncoding,
    /// Returned if the deserializer attempts to deserialize the tag of an enum that is
    /// not in the expected ranges
    InvalidTagEncoding,
    /// Serde has a deserialize_any method that lets the format hint to the
    /// object which route to take in deserializing.
    DeserializeNotSupported,
    /// If (de)serializing a message takes more than the provided size limit, this
    /// error is returned.
    SizeLimit,
    /// Bincode can not encode sequences of unknown length (like iterators).
    SequenceMustHaveLength,
    /// A custom error message from Serde.
    Custom,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            ErrorKind::Io => "Io",
            ErrorKind::InvalidUtf8Encoding => "Invalid UTF-8",
            ErrorKind::InvalidBoolEncoding => "Invalid bool",
            ErrorKind::InvalidCharEncoding => "Invalid char",
            ErrorKind::InvalidTagEncoding => "Invalid tag",
            ErrorKind::DeserializeNotSupported => "Type not supported",
            ErrorKind::SizeLimit => "Size limit reached",
            ErrorKind::SequenceMustHaveLength => "Sequence must have length",
            ErrorKind::Custom => "Custom",
        };
        write!(f, "Deserizalization error: {}", name);
        Ok(())
    }
}

impl ::serde::de::Error for Error {
    fn custom<T: fmt::Display>(_desc: T) -> Error {
        Box::new(ErrorKind::Custom)
    }
}
