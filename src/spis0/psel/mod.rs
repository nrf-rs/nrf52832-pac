#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Pin select for SCK"]
pub struct SCK {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for SCK"]
pub mod sck;
#[doc = "Pin select for MISO signal"]
pub struct MISO {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for MISO signal"]
pub mod miso;
#[doc = "Pin select for MOSI signal"]
pub struct MOSI {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for MOSI signal"]
pub mod mosi;
#[doc = "Pin select for CSN signal"]
pub struct CSN {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for CSN signal"]
pub mod csn;
