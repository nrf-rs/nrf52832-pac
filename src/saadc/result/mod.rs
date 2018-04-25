#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Data pointer"]
pub struct PTR {
    register: VolatileCell<u32>,
}
#[doc = "Data pointer"]
pub mod ptr;
#[doc = "Maximum number of buffer words to transfer"]
pub struct MAXCNT {
    register: VolatileCell<u32>,
}
#[doc = "Maximum number of buffer words to transfer"]
pub mod maxcnt;
#[doc = "Number of buffer words transferred since last START"]
pub struct AMOUNT {
    register: VolatileCell<u32>,
}
#[doc = "Number of buffer words transferred since last START"]
pub mod amount;
