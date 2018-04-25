#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Description cluster[0]: Start address for region 0"]
pub struct START {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Start address for region 0"]
pub mod start;
#[doc = "Description cluster[0]: End address of region 0"]
pub struct END {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: End address of region 0"]
pub mod end;
