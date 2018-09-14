#[doc = "Description cluster[n]: Channel n event end-point"]
pub struct EEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Channel n event end-point"]
pub mod eep;
#[doc = "Description cluster[n]: Channel n task end-point"]
pub struct TEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Channel n task end-point"]
pub mod tep;
