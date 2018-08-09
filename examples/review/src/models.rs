use eos::deserialize::{Deserialize, Reader};
use eos::error::Error;

#[repr(C)]
pub struct Review {
    pub id: u64,
    pub byte1: u64,
    pub byte2: u64,
    pub byte3: u64,
    pub byte4: u64,
}

impl Deserialize for Review {
    fn deserialize(mut d: Reader) -> Result<Self, Error> {
        let id: u64 = d.read_sized()?;
        let byte1: u64 = d.read_sized()?;
        let byte2: u64 = d.read_sized()?;
        let byte3: u64 = d.read_sized()?;
        let byte4: u64 = d.read_sized()?;
        Ok(Review {
            id,
            byte1,
            byte2,
            byte3,
            byte4,
        })
    }
}

#[repr(C)]
pub struct ReadReviewAction {
    pub id: u64,
}

impl Deserialize for ReadReviewAction {
    fn deserialize(mut d: Reader) -> Result<Self, Error> {
        let id: u64 = d.read_sized()?;
        Ok(ReadReviewAction { id })
    }
}
