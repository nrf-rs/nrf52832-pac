#[doc = "Description cluster[n]: Start address for region n"]
pub struct START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Start address for region n"]
pub mod start;
#[doc = "Description cluster[n]: End address of region n"]
pub struct END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: End address of region n"]
pub mod end;
