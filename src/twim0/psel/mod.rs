#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Pin select for SCL signal"]
pub struct SCL {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for SCL signal"]
pub mod scl;
#[doc = "Pin select for SDA signal"]
pub struct SDA {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for SDA signal"]
pub mod sda;
