use eos::{Deserialize, Deserializer, Error};

#[repr(C)]
pub struct HiAction {
    // pub name: u64,
    pub byte1: u8,
}

impl Deserialize for HiAction {
    fn deserialize(mut d: Deserializer) -> Result<Self, Error> {
        let byte1 = d.read_u8()?;
        Ok(HiAction { byte1 })
    }
}
