#[doc = "TXD data pointer"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXD data pointer"]
pub mod ptr;
#[doc = "Maximum number of bytes in transmit buffer"]
pub struct MAXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum number of bytes in transmit buffer"]
pub mod maxcnt;
#[doc = "Number of bytes transmitted in last granted transaction"]
pub struct AMOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of bytes transmitted in last granted transaction"]
pub mod amount;
