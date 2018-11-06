#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Unused."]
    pub unused: UNUSED,
}
#[doc = "Unused."]
pub struct UNUSED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unused."]
pub mod unused;
