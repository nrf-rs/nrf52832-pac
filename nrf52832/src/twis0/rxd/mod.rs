#[doc = "RXD Data pointer"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RXD Data pointer"]
pub mod ptr;
#[doc = "Maximum number of bytes in RXD buffer"]
pub struct MAXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum number of bytes in RXD buffer"]
pub mod maxcnt;
#[doc = "Number of bytes transferred in the last RXD transaction"]
pub struct AMOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of bytes transferred in the last RXD transaction"]
pub mod amount;
