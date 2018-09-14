#[doc = "Data pointer"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data pointer"]
pub mod ptr;
#[doc = "Maximum number of 16-bit samples to be written to output RAM buffer"]
pub struct MAXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum number of 16-bit samples to be written to output RAM buffer"]
pub mod maxcnt;
#[doc = "Number of 16-bit samples written to output RAM buffer since the previous START task"]
pub struct AMOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of 16-bit samples written to output RAM buffer since the previous START task"]
pub mod amount;
