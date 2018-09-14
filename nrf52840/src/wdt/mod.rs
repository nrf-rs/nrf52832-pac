#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the watchdog"]
    pub tasks_start: TASKS_START,
    _reserved1: [u8; 252usize],
    #[doc = "0x100 - Watchdog timeout"]
    pub events_timeout: EVENTS_TIMEOUT,
    _reserved2: [u8; 512usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved4: [u8; 244usize],
    #[doc = "0x400 - Run status"]
    pub runstatus: RUNSTATUS,
    #[doc = "0x404 - Request status"]
    pub reqstatus: REQSTATUS,
    _reserved6: [u8; 252usize],
    #[doc = "0x504 - Counter reload value"]
    pub crv: CRV,
    #[doc = "0x508 - Enable register for reload request registers"]
    pub rren: RREN,
    #[doc = "0x50c - Configuration register"]
    pub config: CONFIG,
    _reserved9: [u8; 240usize],
    #[doc = "0x600 - Description collection[n]: Reload request n"]
    pub rr: [RR; 8],
}
#[doc = "Start the watchdog"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start the watchdog"]
pub mod tasks_start;
#[doc = "Watchdog timeout"]
pub struct EVENTS_TIMEOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog timeout"]
pub mod events_timeout;
#[doc = "Enable interrupt"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Run status"]
pub struct RUNSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Run status"]
pub mod runstatus;
#[doc = "Request status"]
pub struct REQSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request status"]
pub mod reqstatus;
#[doc = "Counter reload value"]
pub struct CRV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter reload value"]
pub mod crv;
#[doc = "Enable register for reload request registers"]
pub struct RREN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable register for reload request registers"]
pub mod rren;
#[doc = "Configuration register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Description collection[n]: Reload request n"]
pub struct RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Reload request n"]
pub mod rr;
