use alloc::string::String;
use eos::{Deserialize, Reader};

#[repr(C)]
pub struct HiAction {
    // pub name: u64,
    pub message: String,
}

impl Deserialize for HiAction {
    type Error = ();
    fn deserialize(mut d: Reader) -> Result<Self, Self::Error> {
        let message = d.read_string();
        Ok(HiAction { message })
    }
}
