#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1280usize],
    #[doc = "0x500 - Enable CRYPTOCELL subsystem"]
    pub enable: ENABLE,
}
#[doc = "Enable CRYPTOCELL subsystem"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable CRYPTOCELL subsystem"]
pub mod enable;
