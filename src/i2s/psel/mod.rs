#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Pin select for MCK signal."]
pub struct MCK {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for MCK signal."]
pub mod mck;
#[doc = "Pin select for SCK signal."]
pub struct SCK {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for SCK signal."]
pub mod sck;
#[doc = "Pin select for LRCK signal."]
pub struct LRCK {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for LRCK signal."]
pub mod lrck;
#[doc = "Pin select for SDIN signal."]
pub struct SDIN {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for SDIN signal."]
pub mod sdin;
#[doc = "Pin select for SDOUT signal."]
pub struct SDOUT {
    register: VolatileCell<u32>,
}
#[doc = "Pin select for SDOUT signal."]
pub mod sdout;
