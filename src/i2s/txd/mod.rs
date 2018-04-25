#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Transmit buffer RAM start address."]
pub struct PTR {
    register: VolatileCell<u32>,
}
#[doc = "Transmit buffer RAM start address."]
pub mod ptr;
