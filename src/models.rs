use eos::{Deserialize, Reader};

#[repr(C)]
pub struct HiAction {
    // pub name: u64,
    pub byte1: u8,
}

impl Deserialize for HiAction {
    type Error = ();
    fn deserialize(mut d: Reader) -> Result<Self, Self::Error> {
        let byte1: u8 = d.read();
        Ok(HiAction { byte1 })
    }
}
