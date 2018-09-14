#[doc = "Description cluster[n]: Write access to region n detected"]
pub struct WA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Write access to region n detected"]
pub mod wa;
#[doc = "Description cluster[n]: Read access to region n detected"]
pub struct RA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Read access to region n detected"]
pub mod ra;
