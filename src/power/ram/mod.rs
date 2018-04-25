#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Description cluster[0]: RAM0 power control register"]
pub struct POWER {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: RAM0 power control register"]
pub mod power;
#[doc = "Description cluster[0]: RAM0 power control set register"]
pub struct POWERSET {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: RAM0 power control set register"]
pub mod powerset;
#[doc = "Description cluster[0]: RAM0 power control clear register"]
pub struct POWERCLR {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: RAM0 power control clear register"]
pub mod powerclr;
