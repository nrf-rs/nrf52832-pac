#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "RXD data pointer"]
pub struct PTR {
    register: VolatileCell<u32>,
}
#[doc = "RXD data pointer"]
pub mod ptr;
#[doc = "Maximum number of bytes in receive buffer"]
pub struct MAXCNT {
    register: VolatileCell<u32>,
}
#[doc = "Maximum number of bytes in receive buffer"]
pub mod maxcnt;
#[doc = "Number of bytes received in last granted transaction"]
pub struct AMOUNT {
    register: VolatileCell<u32>,
}
#[doc = "Number of bytes received in last granted transaction"]
pub mod amount;
