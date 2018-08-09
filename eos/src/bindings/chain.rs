//! Defines API for querying internal chain state
use types::*;

extern "C" {
    /// Gets the set of active producers.
    fn get_active_producers(producers: *mut Opaque, datalen: usize) -> u32;
}

/// Read action
pub fn get_active_producers_set(producers: &mut [AccountName]) -> u32 {
    unsafe { get_active_producers(producers.as_ptr() as *mut Opaque, producers.len()) }
}
