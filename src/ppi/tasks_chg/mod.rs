#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Description cluster[0]: Enable channel group 0"]
pub struct EN {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Enable channel group 0"]
pub mod en;
#[doc = "Description cluster[0]: Disable channel group 0"]
pub struct DIS {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Disable channel group 0"]
pub mod dis;
