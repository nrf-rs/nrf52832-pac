#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "TXD data pointer"]
pub struct PTR {
    register: VolatileCell<u32>,
}
#[doc = "TXD data pointer"]
pub mod ptr;
#[doc = "Maximum number of bytes in transmit buffer"]
pub struct MAXCNT {
    register: VolatileCell<u32>,
}
#[doc = "Maximum number of bytes in transmit buffer"]
pub mod maxcnt;
#[doc = "Number of bytes transmitted in last granted transaction"]
pub struct AMOUNT {
    register: VolatileCell<u32>,
}
#[doc = "Number of bytes transmitted in last granted transaction"]
pub mod amount;
