#[doc = "Data pointer"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data pointer"]
pub mod ptr;
#[doc = "Maximum number of buffer words to transfer"]
pub struct MAXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum number of buffer words to transfer"]
pub mod maxcnt;
#[doc = "Number of buffer words transferred since last START"]
pub struct AMOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of buffer words transferred since last START"]
pub mod amount;
