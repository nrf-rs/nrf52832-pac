#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel group tasks"]
    pub tasks_chg: [TASKS_CHG; 6],
    _reserved0: [u8; 1232usize],
    #[doc = "0x500 - Channel enable register"]
    pub chen: CHEN,
    #[doc = "0x504 - Channel enable set register"]
    pub chenset: CHENSET,
    #[doc = "0x508 - Channel enable clear register"]
    pub chenclr: CHENCLR,
    _reserved1: [u8; 4usize],
    #[doc = "0x510 - PPI Channel"]
    pub ch: [CH; 20],
    _reserved2: [u8; 592usize],
    #[doc = "0x800 - Description collection[0]: Channel group 0"]
    pub chg: [CHG; 6],
    _reserved3: [u8; 248usize],
    #[doc = "0x910 - Fork"]
    pub fork: [FORK; 32],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct TASKS_CHG {
    #[doc = "0x00 - Description cluster[0]: Enable channel group 0"]
    pub en: self::tasks_chg::EN,
    #[doc = "0x04 - Description cluster[0]: Disable channel group 0"]
    pub dis: self::tasks_chg::DIS,
}
#[doc = r" Register block"]
#[doc = "Channel group tasks"]
pub mod tasks_chg;
#[doc = r" Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Description cluster[0]: Channel 0 event end-point"]
    pub eep: self::ch::EEP,
    #[doc = "0x04 - Description cluster[0]: Channel 0 task end-point"]
    pub tep: self::ch::TEP,
}
#[doc = r" Register block"]
#[doc = "PPI Channel"]
pub mod ch;
#[doc = r" Register block"]
#[repr(C)]
pub struct FORK {
    #[doc = "0x00 - Description cluster[0]: Channel 0 task end-point"]
    pub tep: self::fork::TEP,
}
#[doc = r" Register block"]
#[doc = "Fork"]
pub mod fork;
#[doc = "Channel enable register"]
pub struct CHEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable register"]
pub mod chen;
#[doc = "Channel enable set register"]
pub struct CHENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable set register"]
pub mod chenset;
#[doc = "Channel enable clear register"]
pub struct CHENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable clear register"]
pub mod chenclr;
#[doc = "Description collection[0]: Channel group 0"]
pub struct CHG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[0]: Channel group 0"]
pub mod chg;
