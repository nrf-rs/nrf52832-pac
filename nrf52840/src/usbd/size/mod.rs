#[doc = "Description collection[n]: Number of bytes received last in the data stage of this OUT endpoint"]
pub struct EPOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Number of bytes received last in the data stage of this OUT endpoint"]
pub mod epout;
#[doc = "Number of bytes received last on this ISO OUT data endpoint"]
pub struct ISOOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of bytes received last on this ISO OUT data endpoint"]
pub mod isoout;
