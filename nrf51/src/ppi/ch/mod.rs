#[doc = "Channel event end-point."]
pub struct EEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel event end-point."]
pub mod eep;
#[doc = "Channel task end-point."]
pub struct TEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel task end-point."]
pub mod tep;
