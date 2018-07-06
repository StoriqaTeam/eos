use alloc::string::String;
use eos::{Deserialize, Reader};
use error::Error;

#[repr(C)]
pub struct ReviewAction {
    pub user: u64,
    pub hash: String,
    pub mark: i32,
}

impl Deserialize for ReviewAction {
    fn deserialize(mut d: Reader) -> Result<Self, Error> {
        let user: u64 = d.read_sized()?;
        let hash = d.read_string()?;
        let mark: i32 = d.read_sized()?;
        Ok(ReviewAction { user, hash, mark })
    }
}

#[repr(C)]
pub struct ReadReviewAction {
    pub user: u64,
}

impl Deserialize for ReadReviewAction {
    fn deserialize(mut d: Reader) -> Result<Self, Error> {
        let user: u64 = d.read_sized()?;
        Ok(ReadReviewAction { user })
    }
}
