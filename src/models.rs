use alloc::string::String;
use eos::{Deserialize, Reader};

#[repr(C)]
pub struct ReviewAction {
    pub user: u64,
    pub hash: String,
    pub mark: i32,
}

impl Deserialize for ReviewAction {
    type Error = ();
    fn deserialize(mut d: Reader) -> Result<Self, Self::Error> {
        let user: u64 = d.read_primitive();
        let hash = d.read_string();
        let mark: i32 = d.read_primitive();
        Ok(ReviewAction { user, hash, mark })
    }
}
