#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel group tasks."]
    pub tasks_chg: [TASKS_CHG; 4],
    _reserved1: [u8; 1248usize],
    #[doc = "0x500 - Channel enable."]
    pub chen: CHEN,
    #[doc = "0x504 - Channel enable set."]
    pub chenset: CHENSET,
    #[doc = "0x508 - Channel enable clear."]
    pub chenclr: CHENCLR,
    _reserved4: [u8; 4usize],
    #[doc = "0x510 - PPI Channel."]
    pub ch: [CH; 16],
    _reserved5: [u8; 624usize],
    #[doc = "0x800 - Channel group configuration."]
    pub chg: [CHG; 4],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct TASKS_CHG {
    #[doc = "0x00 - Enable channel group."]
    pub en: self::tasks_chg::EN,
    #[doc = "0x04 - Disable channel group."]
    pub dis: self::tasks_chg::DIS,
}
#[doc = r" Register block"]
#[doc = "Channel group tasks."]
pub mod tasks_chg;
#[doc = r" Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Channel event end-point."]
    pub eep: self::ch::EEP,
    #[doc = "0x04 - Channel task end-point."]
    pub tep: self::ch::TEP,
}
#[doc = r" Register block"]
#[doc = "PPI Channel."]
pub mod ch;
#[doc = "Channel enable."]
pub struct CHEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable."]
pub mod chen;
#[doc = "Channel enable set."]
pub struct CHENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable set."]
pub mod chenset;
#[doc = "Channel enable clear."]
pub struct CHENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable clear."]
pub mod chenclr;
#[doc = "Channel group configuration."]
pub struct CHG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel group configuration."]
pub mod chg;
