#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Description cluster[0]: Channel 0 event end-point"]
pub struct EEP {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Channel 0 event end-point"]
pub mod eep;
#[doc = "Description cluster[0]: Channel 0 task end-point"]
pub struct TEP {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Channel 0 task end-point"]
pub mod tep;
