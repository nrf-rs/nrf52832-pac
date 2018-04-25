use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1536usize],
    #[doc = "0x600 - Block protect configuration register 0"]
    pub config0: CONFIG0,
    #[doc = "0x604 - Block protect configuration register 1"]
    pub config1: CONFIG1,
    #[doc = "0x608 - Disable protection mechanism in debug interface mode"]
    pub disableindebug: DISABLEINDEBUG,
    #[doc = "0x60c - Unspecified"]
    pub unused0: UNUSED0,
    #[doc = "0x610 - Block protect configuration register 2"]
    pub config2: CONFIG2,
    #[doc = "0x614 - Block protect configuration register 3"]
    pub config3: CONFIG3,
}
#[doc = "Block protect configuration register 0"]
pub struct CONFIG0 {
    register: VolatileCell<u32>,
}
#[doc = "Block protect configuration register 0"]
pub mod config0;
#[doc = "Block protect configuration register 1"]
pub struct CONFIG1 {
    register: VolatileCell<u32>,
}
#[doc = "Block protect configuration register 1"]
pub mod config1;
#[doc = "Disable protection mechanism in debug interface mode"]
pub struct DISABLEINDEBUG {
    register: VolatileCell<u32>,
}
#[doc = "Disable protection mechanism in debug interface mode"]
pub mod disableindebug;
#[doc = "Unspecified"]
pub struct UNUSED0 {
    register: VolatileCell<u32>,
}
#[doc = "Unspecified"]
pub mod unused0;
#[doc = "Block protect configuration register 2"]
pub struct CONFIG2 {
    register: VolatileCell<u32>,
}
#[doc = "Block protect configuration register 2"]
pub mod config2;
#[doc = "Block protect configuration register 3"]
pub struct CONFIG3 {
    register: VolatileCell<u32>,
}
#[doc = "Block protect configuration register 3"]
pub mod config3;
