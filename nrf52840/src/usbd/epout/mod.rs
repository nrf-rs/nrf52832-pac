#[doc = "Description cluster[n]: Data pointer"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Data pointer"]
pub mod ptr;
#[doc = "Description cluster[n]: Maximum number of bytes to transfer"]
pub struct MAXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Maximum number of bytes to transfer"]
pub mod maxcnt;
#[doc = "Description cluster[n]: Number of bytes transferred in the last transaction"]
pub struct AMOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Number of bytes transferred in the last transaction"]
pub mod amount;
