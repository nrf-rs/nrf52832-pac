#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Pin select for LED signal"]
pub struct LED {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for LED signal"]
pub mod led;
#[doc = "Pin select for A signal"]
pub struct A {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for A signal"]
pub mod a;
#[doc = "Pin select for B signal"]
pub struct B {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for B signal"]
pub mod b;
