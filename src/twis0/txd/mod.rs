#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "TXD Data pointer"]
pub struct PTR {
    register: VolatileCell<u32>,
}
#[doc = "TXD Data pointer"]
pub mod ptr;
#[doc = "Maximum number of bytes in TXD buffer"]
pub struct MAXCNT {
    register: VolatileCell<u32>,
}
#[doc = "Maximum number of bytes in TXD buffer"]
pub mod maxcnt;
#[doc = "Number of bytes transferred in the last TXD transaction"]
pub struct AMOUNT {
    register: VolatileCell<u32>,
}
#[doc = "Number of bytes transferred in the last TXD transaction"]
pub mod amount;
