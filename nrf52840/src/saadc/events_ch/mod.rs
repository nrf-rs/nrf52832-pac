#[doc = "Description cluster[n]: Last result is equal or above CH[n].LIMIT.HIGH"]
pub struct LIMITH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Last result is equal or above CH[n].LIMIT.HIGH"]
pub mod limith;
#[doc = "Description cluster[n]: Last result is equal or below CH[n].LIMIT.LOW"]
pub struct LIMITL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Last result is equal or below CH[n].LIMIT.LOW"]
pub mod limitl;
