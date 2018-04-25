#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Description cluster[0]: Input positive pin selection for CH[0]"]
pub struct PSELP {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Input positive pin selection for CH[0]"]
pub mod pselp;
#[doc = "Description cluster[0]: Input negative pin selection for CH[0]"]
pub struct PSELN {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Input negative pin selection for CH[0]"]
pub mod pseln;
#[doc = "Description cluster[0]: Input configuration for CH[0]"]
pub struct CONFIG {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Input configuration for CH[0]"]
pub mod config;
#[doc = "Description cluster[0]: High/low limits for event monitoring a channel"]
pub struct LIMIT {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: High/low limits for event monitoring a channel"]
pub mod limit;
