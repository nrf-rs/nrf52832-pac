#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Description cluster[0]: Last results is equal or above CH[0].LIMIT.HIGH"]
pub struct LIMITH {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Last results is equal or above CH[0].LIMIT.HIGH"]
pub mod limith;
#[doc = "Description cluster[0]: Last results is equal or below CH[0].LIMIT.LOW"]
pub struct LIMITL {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Last results is equal or below CH[0].LIMIT.LOW"]
pub mod limitl;
