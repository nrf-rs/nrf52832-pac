#[doc = "Description cluster\\[0\\]: Channel 0 event end-point"]
pub struct EEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster\\[0\\]: Channel 0 event end-point"]
pub mod eep;
#[doc = "Description cluster\\[0\\]: Channel 0 task end-point"]
pub struct TEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster\\[0\\]: Channel 0 task end-point"]
pub mod tep;
