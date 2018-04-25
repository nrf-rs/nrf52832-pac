#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Receive buffer RAM start address."]
pub struct PTR {
    register: VolatileCell<u32>,
}
#[doc = "Receive buffer RAM start address."]
pub mod ptr;
