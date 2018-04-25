#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Configuration of outgoing frames"]
pub struct FRAMECONFIG {
    register: VolatileCell<u32>,
}
#[doc = "Configuration of outgoing frames"]
pub mod frameconfig;
#[doc = "Size of outgoing frame"]
pub struct AMOUNT {
    register: VolatileCell<u32>,
}
#[doc = "Size of outgoing frame"]
pub mod amount;
