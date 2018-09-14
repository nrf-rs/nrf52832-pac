#[doc = "Description cluster[n]: Input positive pin selection for CH[n]"]
pub struct PSELP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Input positive pin selection for CH[n]"]
pub mod pselp;
#[doc = "Description cluster[n]: Input negative pin selection for CH[n]"]
pub struct PSELN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Input negative pin selection for CH[n]"]
pub mod pseln;
#[doc = "Description cluster[n]: Input configuration for CH[n]"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Input configuration for CH[n]"]
pub mod config;
#[doc = "Description cluster[n]: High/low limits for event monitoring of a channel"]
pub struct LIMIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: High/low limits for event monitoring of a channel"]
pub mod limit;
