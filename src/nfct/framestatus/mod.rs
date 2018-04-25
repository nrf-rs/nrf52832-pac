#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Result of last incoming frames"]
pub struct RX {
    register: VolatileCell<u32>,
}
#[doc = "Result of last incoming frames"]
pub mod rx;
