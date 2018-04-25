#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Pin select for SCK"]
pub struct SCK {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for SCK"]
pub mod sck;
#[doc = "Pin select for MOSI"]
pub struct MOSI {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for MOSI"]
pub mod mosi;
#[doc = "Pin select for MISO"]
pub struct MISO {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for MISO"]
pub mod miso;
