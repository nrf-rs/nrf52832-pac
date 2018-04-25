#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "RAM address pointer to write samples to with EasyDMA"]
pub struct PTR {
    register: VolatileCell<u32>,
}
#[doc = "RAM address pointer to write samples to with EasyDMA"]
pub mod ptr;
#[doc = "Number of samples to allocate memory for in EasyDMA mode"]
pub struct MAXCNT {
    register: VolatileCell<u32>,
}
#[doc = "Number of samples to allocate memory for in EasyDMA mode"]
pub mod maxcnt;
