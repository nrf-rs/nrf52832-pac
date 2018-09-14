#[doc = "Flash memory source address"]
pub struct SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash memory source address"]
pub mod src;
#[doc = "RAM destination address"]
pub struct DST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RAM destination address"]
pub mod dst;
#[doc = "Read transfer length"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read transfer length"]
pub mod cnt;
