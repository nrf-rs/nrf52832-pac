#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Description cluster[0]: Reserved for future use"]
pub struct START {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Reserved for future use"]
pub mod start;
#[doc = "Description cluster[0]: Reserved for future use"]
pub struct END {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Reserved for future use"]
pub mod end;
#[doc = "Description cluster[0]: Subregions of region 0"]
pub struct SUBS {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Subregions of region 0"]
pub mod subs;
