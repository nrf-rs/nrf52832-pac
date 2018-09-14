#[doc = "Data pointer"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data pointer"]
pub mod ptr;
#[doc = "Maximum number of bytes in receive buffer"]
pub struct MAXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum number of bytes in receive buffer"]
pub mod maxcnt;
#[doc = "Number of bytes transferred in the last transaction"]
pub struct AMOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of bytes transferred in the last transaction"]
pub mod amount;
#[doc = "EasyDMA list type"]
pub struct LIST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EasyDMA list type"]
pub mod list;
