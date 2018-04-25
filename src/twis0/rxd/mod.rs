#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "RXD Data pointer"]
pub struct PTR {
    register: VolatileCell<u32>,
}
#[doc = "RXD Data pointer"]
pub mod ptr;
#[doc = "Maximum number of bytes in RXD buffer"]
pub struct MAXCNT {
    register: VolatileCell<u32>,
}
#[doc = "Maximum number of bytes in RXD buffer"]
pub mod maxcnt;
#[doc = "Number of bytes transferred in the last RXD transaction"]
pub struct AMOUNT {
    register: VolatileCell<u32>,
}
#[doc = "Number of bytes transferred in the last RXD transaction"]
pub mod amount;
