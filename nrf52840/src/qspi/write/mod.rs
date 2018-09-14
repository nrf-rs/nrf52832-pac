#[doc = "Flash destination address"]
pub struct DST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash destination address"]
pub mod dst;
#[doc = "RAM source address"]
pub struct SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RAM source address"]
pub mod src;
#[doc = "Write transfer length"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write transfer length"]
pub mod cnt;
