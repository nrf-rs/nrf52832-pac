#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Description cluster[0]: Beginning address in Data RAM of this sequence"]
pub struct PTR {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Beginning address in Data RAM of this sequence"]
pub mod ptr;
#[doc = "Description cluster[0]: Amount of values (duty cycles) in this sequence"]
pub struct CNT {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Amount of values (duty cycles) in this sequence"]
pub mod cnt;
#[doc = "Description cluster[0]: Amount of additional PWM periods between samples loaded into compare register"]
pub struct REFRESH {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Amount of additional PWM periods between samples loaded into compare register"]
pub mod refresh;
#[doc = "Description cluster[0]: Time added after the sequence"]
pub struct ENDDELAY {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Time added after the sequence"]
pub mod enddelay;
