#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Configuration of incoming frames"]
pub struct FRAMECONFIG {
    register: VolatileCell<u32>,
}
#[doc = "Configuration of incoming frames"]
pub mod frameconfig;
#[doc = "Size of last incoming frame"]
pub struct AMOUNT {
    register: VolatileCell<u32>,
}
#[doc = "Size of last incoming frame"]
pub mod amount;
