#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Description cluster[0]: Write access to region 0 detected"]
pub struct WA {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Write access to region 0 detected"]
pub mod wa;
#[doc = "Description cluster[0]: Read access to region 0 detected"]
pub struct RA {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Read access to region 0 detected"]
pub mod ra;
