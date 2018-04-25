#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Size of RXD and TXD buffers."]
pub struct MAXCNT {
    register: VolatileCell<u32>,
}
#[doc = "Size of RXD and TXD buffers."]
pub mod maxcnt;
